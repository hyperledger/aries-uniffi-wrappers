use super::error::ErrorCode;
use super::requests::Request;
use super::POOL_CONFIG;
use indy_vdr::common::error::VdrResult;
use indy_vdr::pool::{
    PoolBuilder, PoolRunner, PoolTransactions, RequestMethod, RequestResult, TimingResult,
};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{oneshot, RwLock};

pub struct Pool {
    pool: RwLock<Option<PoolRunner>>,
}

#[uniffi::export]
fn open_pool(
    transactions_path: Option<String>,
    transactions: Option<String>,
    node_weights: Option<HashMap<String, f32>>,
) -> Result<Arc<Pool>, ErrorCode> {
    let txns = if let Some(txns) = transactions {
        PoolTransactions::from_json(txns.as_str())?
    } else if let Some(path) = transactions_path {
        PoolTransactions::from_json_file(path.as_str())?
    } else {
        return Err(ErrorCode::Input {
            message:
                "Invalid pool create parameters: must provide transactions or transactions_path"
                    .to_string(),
        });
    };

    let builder = {
        let gcfg = read_lock!(POOL_CONFIG)?;
        PoolBuilder::from(gcfg.clone())
            .transactions(txns)?
            .node_weights(node_weights)
    };
    let pool = builder.into_runner()?;
    Ok(Arc::new(Pool {
        pool: RwLock::new(Some(pool)),
    }))
}

fn handle_request_result(
    result: VdrResult<(RequestResult<String>, Option<TimingResult>)>,
) -> (ErrorCode, String) {
    match result {
        Ok((reply, _timing)) => match reply {
            RequestResult::Reply(body) => (ErrorCode::Success {}, body),
            RequestResult::Failed(err) => {
                let code = ErrorCode::from(err);
                (code, String::new())
            }
        },
        Err(err) => {
            let code = ErrorCode::from(err);
            (code, String::new())
        }
    }
}

#[uniffi::export(async_runtime = "tokio")]
impl Pool {
    pub async fn get_status(&self) -> Result<String, ErrorCode> {
        let (tx, rx) = oneshot::channel();
        read_pool!(self.pool)?.get_status(Box::new(move |result| {
            let (errcode, reply) = match result {
                Ok(status) => {
                    let status = status.serialize().unwrap();
                    (ErrorCode::Success {}, status)
                }
                Err(err) => {
                    let code = ErrorCode::from(err);
                    (code, String::new())
                }
            };
            let _ = tx.send((errcode, reply));
        }))?;
        let (errcode, reply) = rx.await.map_err(|err| ErrorCode::Unexpected {
            message: format!("Channel error: {}", err),
        })?;
        if errcode != (ErrorCode::Success {}) {
            return Err(errcode);
        }
        Ok(reply)
    }

    pub async fn get_transactions(&self) -> Result<String, ErrorCode> {
        let (tx, rx) = oneshot::channel();
        read_pool!(self.pool)?.get_transactions(Box::new(move |result| {
            let (errcode, reply) = match result {
                Ok(txns) => (ErrorCode::Success {}, txns.join("\n")),
                Err(err) => {
                    let code = ErrorCode::from(err);
                    (code, String::new())
                }
            };
            let _ = tx.send((errcode, reply));
        }))?;
        let (errcode, reply) = rx.await.map_err(|err| ErrorCode::Unexpected {
            message: format!("Channel error: {}", err),
        })?;
        if errcode != (ErrorCode::Success {}) {
            return Err(errcode);
        }
        Ok(reply)
    }

    pub async fn submit_action(
        &self,
        request: Arc<Request>,
        node_aliases: Option<Vec<String>>,
        timeout: Option<i64>,
    ) -> Result<String, ErrorCode> {
        request.set_method(RequestMethod::Full {
            node_aliases,
            timeout,
        })?;
        let req = take_req!(request.req)?;
        let (tx, rx) = oneshot::channel();
        read_pool!(self.pool)?.send_request(
            req,
            Box::new(move |result| {
                let (errcode, reply) = handle_request_result(result);
                let _ = tx.send((errcode, reply));
            }),
        )?;
        let (errcode, reply) = rx.await.map_err(|err| ErrorCode::Unexpected {
            message: format!("Channel error: {}", err),
        })?;
        if errcode != (ErrorCode::Success {}) {
            return Err(errcode);
        }
        Ok(reply)
    }

    pub async fn submit_request(&self, request: Arc<Request>) -> Result<String, ErrorCode> {
        let req = take_req!(request.req)?;
        let (tx, rx) = oneshot::channel();
        read_pool!(self.pool)?.send_request(
            req,
            Box::new(move |result| {
                let (errcode, reply) = handle_request_result(result);
                let _ = tx.send((errcode, reply));
            }),
        )?;
        let (errcode, reply) = rx.await.map_err(|err| ErrorCode::Unexpected {
            message: format!("Channel error: {}", err),
        })?;
        if errcode != (ErrorCode::Success {}) {
            return Err(errcode);
        }
        Ok(reply)
    }

    pub async fn close(&self) -> Result<(), ErrorCode> {
        _ = self.pool.write().await.take();
        Ok(())
    }
}

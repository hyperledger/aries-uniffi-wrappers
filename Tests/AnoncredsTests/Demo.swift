import XCTest
@testable import Anoncreds

final class Demo: XCTestCase {
    let issuer = Issuer()
    let prover = Prover()
    let verifier = Verifier()

    func testDemo() throws {
        let issuer_id   = "TZQuLp43UcYTdtc3HewcDz"
        let entropy     = "entropy"
        let rev_idx = UInt32(1)

        let schema = try issuer.createSchema(
            schemaName: "schema-\(UUID().uuidString)",
            schemaVersion: "1.0",
            issuerId: issuer_id,
            attrNames: ["name","age","sex","height"])
        let schema_id   = schema.schemaId()
        print("schema: \(schema.toJson())")
        print("schema_id: \(schema_id)")

        let credDefTuple = try issuer.createCredentialDefinition(
            schemaId: schema_id,
            schema: schema,
            tag: "tag",
            issuerId: issuer_id,
            supportRevocation: true)
        let cred_def_id = credDefTuple.credDef.credDefId()
        print("cred_def: \(credDefTuple.credDef.toJson())")
        print("cred_def_id: \(cred_def_id)")

        let regDefTuple = try issuer.createRevocationRegistryDef(
            credDef: credDefTuple.credDef,
            credDefId: cred_def_id,
            tag: "some_tag",
            maxCredNum: 10,
            tailsDirPath: nil)
        let rev_reg_id  = regDefTuple.revRegDef.revRegId()
        print("rev_reg_def: \(regDefTuple.revRegDef.toJson())")
        print("rev_reg_def_id: \(rev_reg_id)")

        let time_create_rev_status_list = UInt64(12)
        let revocation_status_list = try issuer.createRevocationStatusList(
            credDef: credDefTuple.credDef,
            revRegDefId: rev_reg_id,
            revRegDef: regDefTuple.revRegDef,
            revRegPriv: regDefTuple.revRegDefPriv,
            timestamp: time_create_rev_status_list,
            issuanceByDefault: true)

        let link_secret = try createLinkSecret()
        let link_secret_id = "default"
        
        let cred_offer = try issuer.createCredentialOffer(
            schemaId: schema_id,
            credDefId: cred_def_id,
            keyProof: credDefTuple.keyCorrectnessProof)

        let credReqTuple = try prover.createCredentialRequest(
            entropy: entropy,
            proverDid: nil,
            credDef: credDefTuple.credDef,
            linkSecret: link_secret,
            linkSecretId: link_secret_id,
            credOffer: cred_offer)

        let issue_cred = try issuer.createCredential(
            credDef: credDefTuple.credDef,
            credDefPrivate: credDefTuple.credDefPriv,
            credOffer: cred_offer,
            credRequest: credReqTuple.request,
            attrRawValues: ["sex": "male", "name": "Alex", "height": "175", "age": "28"],
            attrEncValues: nil,
            revocationConfig: CredentialRevocationConfig(
                regDef: regDefTuple.revRegDef,
                regDefPrivate: regDefTuple.revRegDefPriv,
                statusList: revocation_status_list,
                registryIndex: rev_idx))

        let recv_cred = try prover.processCredential(
            cred: issue_cred,
            credReqMetadata: credReqTuple.metadata,
            linkSecret: link_secret,
            credDef: credDefTuple.credDef,
            revRegDef: regDefTuple.revRegDef)

        let time_after_creating_cred = time_create_rev_status_list + 1
        let issued_rev_status_list = try issuer.updateRevocationStatusList(
            credDef: credDefTuple.credDef,
            timestamp: time_after_creating_cred,
            issued: [rev_idx],
            revoked: nil,
            revRegDef: regDefTuple.revRegDef,
            revRegPriv: regDefTuple.revRegDefPriv,
            currentList: revocation_status_list)

        let nonce = try verifier.generateNonce()
        let pres_req = try PresentationRequest(json: """
            {
                "nonce":"\(nonce)",
                "name":"pres_req_1",
                "version":"0.1",
                 "requested_attributes":{
                    "attr1_referent":{
                        "name":"name",
                        "issuer_id": "\(issuer_id)"
                    },
                    "attr2_referent":{
                        "name":"sex"
                    },
                    "attr3_referent":{"name":"phone"},
                    "attr4_referent":{
                        "names": ["name", "height"]
                    }
                },
                "requested_predicates":{
                    "predicate1_referent":{"name":"age","p_type":">=","p_value":18}
                },
                "non_revoked": {"from": 10, "to": 200}
                    
            }
        """)

        var rev_state = try prover.createOrUpdateRevocationState(
            revRegDef: regDefTuple.revRegDef,
            revStatusList: revocation_status_list,
            revRegIdx: rev_idx,
            tailsPath: regDefTuple.revRegDef.tailsLocation(),
            revState: nil,
            oldRevStatusList: nil)

        let schemas = [schema_id: schema]
        let cred_defs = [cred_def_id: credDefTuple.credDef]
        let rev_reg_defs = [rev_reg_id: regDefTuple.revRegDef]
        var rev_status_lists = [issued_rev_status_list]

        var present = RequestedCredential(
            cred: recv_cred,
            timestamp: time_after_creating_cred,
            revState: rev_state,
            requestedAttributes: ["attr1_referent": true, "attr2_referent": false, "attr4_referent": true],
            requestedPredicates: ["predicate1_referent"])

        var presentation = try prover.createPresentation(
            presReq: pres_req,
            requestedCredentials: [present],
            selfAttestedAttributes: ["attr3_referent": "8-800-300"],
            linkSecret: link_secret,
            schemas: schemas,
            credDefs: cred_defs)

        var verified = try verifier.verifyPresentation(
            presentation: presentation,
            presReq: pres_req,
            schemas: schemas,
            credDefs: cred_defs,
            revRegDefs: rev_reg_defs,
            revStatusLists: rev_status_lists,
            nonrevokeIntervalOverride: nil)

        XCTAssertTrue(verified, "verify result should be true")

        let time_revoke_cred = time_after_creating_cred + 1
        let revoked_status_list = try issuer.updateRevocationStatusList(
            credDef: credDefTuple.credDef,
            timestamp: time_revoke_cred,
            issued: nil,
            revoked: [rev_idx],
            revRegDef: regDefTuple.revRegDef,
            revRegPriv: regDefTuple.revRegDefPriv,
            currentList: issued_rev_status_list)

        rev_status_lists.append(revoked_status_list)

        rev_state = try prover.createOrUpdateRevocationState(
            revRegDef: regDefTuple.revRegDef,
            revStatusList: revocation_status_list,
            revRegIdx: rev_idx,
            tailsPath: regDefTuple.revRegDef.tailsLocation(),
            revState: rev_state,
            oldRevStatusList: revoked_status_list)

        present.timestamp = time_revoke_cred
        present.revState = rev_state

        presentation = try prover.createPresentation(
            presReq: pres_req,
            requestedCredentials: [present],
            selfAttestedAttributes: ["attr3_referent": "8-800-300"],
            linkSecret: link_secret,
            schemas: schemas,
            credDefs: cred_defs)

        verified = try verifier.verifyPresentation(
            presentation: presentation,
            presReq: pres_req,
            schemas: schemas,
            credDefs: cred_defs,
            revRegDefs: rev_reg_defs,
            revStatusLists: rev_status_lists,
            nonrevokeIntervalOverride: nil)

        XCTAssertFalse(verified, "verify result should be false")
    }
}

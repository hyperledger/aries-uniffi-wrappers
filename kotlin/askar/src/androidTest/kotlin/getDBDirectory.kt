package org.hyperledger.askar_uniffi

import okio.FileSystem

actual fun getDBDirectory(): String {
    return FileSystem.SYSTEM_TEMPORARY_DIRECTORY.toString()
}
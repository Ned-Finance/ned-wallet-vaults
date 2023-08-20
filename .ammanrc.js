module.exports = {
    validator: {
        killRunningValidators: true,
        programs: [],
        accounts: [
            {
                label: "Token Program",
                accountId: "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
                executable: true,
                cluster: "https://rpc.helius.xyz/?api-key=be1f775e-5cb5-4e93-8f5d-02a2cf9b2261",
            },
            {
                label: "Meteora",
                accountId: "24Uqj9JCLxUeoC3hGfh5W3s9FM9uCHDS2SG3LYwBpyTi",
                executable: true,
                cluster: "https://rpc.helius.xyz/?api-key=be1f775e-5cb5-4e93-8f5d-02a2cf9b2261",
            },
            {
                label: "Meteora USDC Vault",
                accountId: "3ESUFCnRNgZ7Mn2mPPUMmXYaKU8jpnV9VtA17M7t2mHQ",
                executable: true,
                cluster: "https://rpc.helius.xyz/?api-key=be1f775e-5cb5-4e93-8f5d-02a2cf9b2261",
            },
            {
                label: "Meteora USDC Vault LP Mint",
                accountId: "3RpEekjLE5cdcG15YcXJUpxSepemvq2FpmMcgo342BwC",
                executable: true,
                cluster: "https://rpc.helius.xyz/?api-key=be1f775e-5cb5-4e93-8f5d-02a2cf9b2261",
            },
            {
                label: "Meteora USDC Token Vault (strategy)",
                accountId: "B4p3pLrkuig6bscb37VYmFvakqjeXoXxtkRvP4jmmsYj",
                executable: true,
                cluster: "https://rpc.helius.xyz/?api-key=be1f775e-5cb5-4e93-8f5d-02a2cf9b2261",
            },
            {
                label: "USD Coin",
                accountId: "4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU",
                cluster: "https://rpc.helius.xyz/?api-key=be1f775e-5cb5-4e93-8f5d-02a2cf9b2261",
            },
            {
                label: "Wrapped Sol",
                accountId: "So11111111111111111111111111111111111111112",
                cluster: "https://rpc.helius.xyz/?api-key=be1f775e-5cb5-4e93-8f5d-02a2cf9b2261",
            },
            {
                label: "Account",
                accountId: "AJBbXVqxBAhLHsQvasXnn58aJTjZixKeAsW1KnPeraDs",
                cluster: "https://rpc.helius.xyz/?api-key=be1f775e-5cb5-4e93-8f5d-02a2cf9b2261",
            },
        ],
        jsonRpcUrl: "127.0.0.1",
        websocketUrl: "",
        commitment: "confirmed",
        ledgerDir: "./test-ledger",
        resetLedger: true,
        verifyFees: false,
        detached: false,
    },
    relay: {
        enabled: true,
        killlRunningRelay: true,
    },
    storage: {
        enabled: true,
        storageId: "mock-storage",
        clearOnStart: true,
    },
};

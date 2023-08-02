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
                label: "USD Coin",
                accountId: "4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU",
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

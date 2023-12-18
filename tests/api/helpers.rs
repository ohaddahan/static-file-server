pub struct TestApp {
    pub cli_args: CliArgs,
    pub api_client: reqwest::Client,
}

impl TestApp {
    pub async fn spawn_app(cli_args: &CliArgs) -> TestApp {
        let app = tokio::spawn(spawn_server(&args));
        let api_client = reqwest::Client::builder()
            .redirect(reqwest::redirect::Policy::none())
            .cookie_store(true)
            .build()
            .unwrap();
        let cwd = env::current_dir().unwrap();
        let mnemonic_path = cwd.join("fixtures/eth_wallet.mnemonic");
        let wallet = MnemonicBuilder::<English>::default()
            .phrase(mnemonic_path.clone())
            .index(0u32)
            .unwrap()
            .build()
            .unwrap();
        let wallet_2 = MnemonicBuilder::<English>::default()
            .phrase(mnemonic_path.clone())
            .index(1u32)
            .unwrap()
            .build()
            .unwrap();
        let wallet_3 = MnemonicBuilder::<English>::default()
            .phrase(mnemonic_path.clone())
            .index(2u32)
            .unwrap()
            .build()
            .unwrap();
        assert_eq!(wallet.address(), WALLET_ADDRESS.parse::<Address>().unwrap());

        let contract = deploy_erc20(&provider, &wallet).await.unwrap();

        TestApp {
            address,
            port,
            db_pool,
            api_client,
            configuration,
            provider,
            wallets: vec![wallet, wallet_2, wallet_3],
            contract,
            configs,
        }
    }
}

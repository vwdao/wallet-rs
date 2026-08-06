// demo_tron_grpc_client 通过钱包链网关的 gRPC 代理访问 TRON 上游。
//
// 用法:
//
//	go run . -endpoint grpc://127.0.0.1:50051 -key gw_xxxx
//
// 说明:
//   - endpoint 是网关 grpc_listen 地址（明文 h2c），不是 HTTP JSON-RPC 的 :8545。
//   - key 通过 gRPC metadata (TRON-PRO-API-KEY / x-api-key) 传给网关做鉴权，
//     URL 路径里的 /rpc/tron/gw_xxx 对 gRPC 客户端无效。
//   - 网关会把 /protocol.Wallet/* 请求原样转发到 chain 195 的 grpc 端点；
//     某个方法是否可用取决于上游是否注册了该方法。
package main

import (
	"crypto/tls"
	"encoding/hex"
	"flag"
	"fmt"
	"strings"
	"time"

	"github.com/fbsobreira/gotron-sdk/pkg/client"
	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials"
	"google.golang.org/grpc/credentials/insecure"
)

func dialTarget(endpoint string) (address string, useTLS bool) {
	switch {
	case strings.HasPrefix(endpoint, "grpcs://"):
		return strings.TrimPrefix(endpoint, "grpcs://"), true
	case strings.HasPrefix(endpoint, "grpc://"):
		return strings.TrimPrefix(endpoint, "grpc://"), false
	default:
		return endpoint, false
	}
}

func step(name string, fn func() error) {
	fmt.Printf("\n=== %s ===\n", name)
	if err := fn(); err != nil {
		fmt.Printf("  ERROR: %v\n", err)
		return
	}
	fmt.Printf("  OK\n")
}

func main() {
	endpoint := flag.String("endpoint", "grpc://127.0.0.1:50051", "gateway gRPC endpoint (grpc:// or grpcs://)")
	apiKey := flag.String("key", "gw_5a6ce3f0795246bc9d5b938e5af8a399", "chain-gateway api key, sent as TRON-PRO-API-KEY metadata")
	hash := flag.String("hash", "a66fc4ad2f3390cf605f99512c3b002a3354ea74d761fc803c454246cebcfa40", "transaction hash for GetTransactionByID")
	owner := flag.String("address", "TR7NHqjeKQxGTCi8q8ZY4pL8otSzgjLj6t", "wallet address for TRC20 balance")
	contract := flag.String("contract", "TR7NHqjeKQxGTCi8q8ZY4pL8otSzgjLj6t", "TRC20 contract address")
	timeout := flag.Duration("timeout", 30*time.Second, "per-call timeout")
	gap := flag.Duration("gap", 3*time.Second, "delay between calls (upstream rate-limits)")
	flag.Parse()

	address, useTLS := dialTarget(*endpoint)

	c := client.NewGrpcClientWithTimeout(address, *timeout)
	var creds grpc.DialOption
	if useTLS {
		creds = grpc.WithTransportCredentials(credentials.NewTLS(&tls.Config{}))
	} else {
		creds = grpc.WithTransportCredentials(insecure.NewCredentials())
	}
	if err := c.Start(creds); err != nil {
		fmt.Printf("connect %s failed: %v\n", address, err)
		return
	}
	defer c.Stop()
	if *apiKey != "" {
		_ = c.SetAPIKey(*apiKey)
	}
	fmt.Printf("dialed %s (tls=%v), api key=%s...\n", address, useTLS, shorten(*apiKey, 8))

	step("GetNowBlock", func() error {
		block, err := c.GetNowBlock()
		if err != nil {
			return err
		}
		fmt.Printf("  number=%d blockid=%s\n",
			block.BlockHeader.GetRawData().GetNumber(),
			hex.EncodeToString(block.Blockid))
		return nil
	})
	time.Sleep(*gap)

	step("GetTransactionByID", func() error {
		tx, err := c.GetTransactionByID(*hash)
		if err != nil {
			return err
		}
		if tx.RawData == nil {
			return fmt.Errorf("tx has no raw_data")
		}
		fmt.Printf("  contracts=%d\n", len(tx.RawData.GetContract()))
		for i, contract := range tx.RawData.GetContract() {
			fmt.Printf("  contract[%d] type=%s\n", i, contract.GetType())
		}
		return nil
	})
	time.Sleep(*gap)

	step("TRC20ContractBalance", func() error {
		balance, err := c.TRC20ContractBalance(*owner, *contract)
		if err != nil {
			return err
		}
		fmt.Printf("  balance=%s\n", balance.String())
		return nil
	})
	time.Sleep(*gap)

	step("GetEnergyPrices", func() error {
		prices, err := c.GetEnergyPrices()
		if err != nil {
			return err
		}
		fmt.Printf("  prices=%s\n", prices.GetPrices())
		return nil
	})

	fmt.Println("\ndone")
}

func shorten(s string, n int) string {
	if len(s) <= n {
		return s
	}
	return s[:n] + "..."
}

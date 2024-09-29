/**
 *  Copyright 2024 dqx0
 *
 *  Licensed under the Apache License, Version 2.0 (the "License");
 *  you may not use this file except in compliance with the License.
 *  You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 *  Unless required by applicable law or agreed to in writing, software
 *  distributed under the License is distributed on an "AS IS" BASIS,
 *  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 *  See the License for the specific language governing permissions and
 *  limitations under the License.
 *
 */

package main

import (
	"context"
	"log"
	"time"

	auth "github.com/dqx0/totp-server/pb/v1"
	"github.com/pquerna/otp/totp"
	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials/insecure"
)

func main() {
	conn, err := grpc.Dial(":50051", grpc.WithTransportCredentials(insecure.NewCredentials()), grpc.WithBlock())
	if err != nil {
		log.Fatalf("did not connect: %v", err)
	}
	defer conn.Close()

	client := auth.NewAuthServiceClient(conn)

	generateTOTPRequest := &auth.GenerateTOTPRequest{
		AccountName: "user@example.com",
	}

	ctx, cancel := context.WithTimeout(context.Background(), time.Second)
	defer cancel()

	generateResp, err := client.GenerateTOTP(ctx, generateTOTPRequest)
	if err != nil {
		log.Fatalf("could not generate TOTP: %v", err)
	}

	token, err := totp.GenerateCode(generateResp.Key, time.Now().UTC())
	if err != nil {
		log.Fatalf("could not generate TOTP code: %v", err)
	}

	validateTOTPRequest := &auth.ValidateTOTPRequest{
		Token:  token,
		Secret: generateResp.Key,
	}

	validateResp, err := client.ValidateTOTP(ctx, validateTOTPRequest)
	if err != nil {
		log.Fatalf("could not validate TOTP: %v", err)
	}
	log.Printf("TOTP is valid: %s", result(validateResp.Valid))
}

func result(valid bool) string {
	if valid {
		return "\033[1;32mTRUE\033[0m\n"
	}
	return "\033[1;31mFALSE\033[0m\n"
}

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
	"net"

	auth "github.com/dqx0/totp-server/pb/v1"
	"github.com/pquerna/otp/totp"
	"google.golang.org/grpc"
)

type AuthServiceServer struct {
	auth.UnimplementedAuthServiceServer
}

func (s *AuthServiceServer) GenerateTOTP(ctx context.Context, req *auth.GenerateTOTPRequest) (*auth.GenerateTOTPResponse, error) {
	key, err := totp.Generate(totp.GenerateOpts{
		Issuer:      "MyApp",
		AccountName: req.AccountName,
	})
	if err != nil {
		return nil, err
	}
	log.Printf("Generated TOTP key for %s: %s", req.AccountName, key.Secret())
	return &auth.GenerateTOTPResponse{Key: key.Secret()}, nil
}

func (s *AuthServiceServer) ValidateTOTP(ctx context.Context, req *auth.ValidateTOTPRequest) (*auth.ValidateTOTPResponse, error) {
	log.Printf("Validating TOTP token for %s", req.Token)
	valid := totp.Validate(req.Token, req.Secret)
	return &auth.ValidateTOTPResponse{Valid: valid}, nil
}

func main() {
	lis, err := net.Listen("tcp", ":50051")
	if err != nil {
		log.Fatalf("failed to listen: %v", err)
	}

	grpcServer := grpc.NewServer()
	auth.RegisterAuthServiceServer(grpcServer, &AuthServiceServer{})

	log.Printf("gRPC server listening on :50051")
	if err := grpcServer.Serve(lis); err != nil {
		log.Fatalf("failed to serve: %v", err)
	}
}

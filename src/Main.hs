module Main where

import MyLib

main :: IO ()
main = do
  client <- createClient "esdb://localhost:2113?tls=false"
  putStrLn "Successfully started the Rust gRPC client"
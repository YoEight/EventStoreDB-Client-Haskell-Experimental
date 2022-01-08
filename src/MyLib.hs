module MyLib (printString, createClient, Client) where

import Foreign
import Foreign.C.String
import Foreign.C.Types

data EsEnv

newtype Client = Client (Ptr EsEnv)

foreign import ccall unsafe "print_string" cc_printString :: CString -> IO ()

foreign import ccall unsafe "create_es_env" cc_createEsEnv :: CString -> IO (Ptr EsEnv)

printString :: String -> IO ()
printString s = do
  cs <- newCString s
  cc_printString cs

createClient :: String -> IO Client
createClient s = do
  cs <- newCString s
  env <- cc_createEsEnv cs
  pure (Client env)
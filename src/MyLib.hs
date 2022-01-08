module MyLib (printString) where

import Foreign.C.String
import Foreign.C.Types

foreign import ccall unsafe "print_string" cc_printString :: CString -> IO ()

printString :: String -> IO ()
printString s = do
  cs <- newCString s
  cc_printString cs
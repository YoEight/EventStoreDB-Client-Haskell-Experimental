import Data.Maybe (fromJust)
import qualified Distribution.PackageDescription as PkgDesc
import Distribution.Simple (Args, confHook, defaultMainWithHooks, simpleUserHooks)
import Distribution.Simple.LocalBuildInfo (LocalBuildInfo, localPkgDescr)
import Distribution.Simple.Setup (ConfigFlags)
import System.Directory (getCurrentDirectory)

main :: IO ()
main =
  defaultMainWithHooks
    simpleUserHooks
      { confHook = rustyConfHook
      }

rustyConfHook :: (PkgDesc.GenericPackageDescription, PkgDesc.HookedBuildInfo) -> ConfigFlags -> IO LocalBuildInfo
rustyConfHook (description, buildInfo) flags = do
  localBuildInfo <- confHook simpleUserHooks (description, buildInfo) flags
  let packageDescription = localPkgDescr localBuildInfo
      library = fromJust $ PkgDesc.library packageDescription
      libraryBuildInfo = PkgDesc.libBuildInfo library

  dir <- getCurrentDirectory

  return
    localBuildInfo
      { localPkgDescr =
          packageDescription
            { PkgDesc.library =
                Just $
                  library
                    { PkgDesc.libBuildInfo =
                        libraryBuildInfo
                          { PkgDesc.extraLibDirs = (dir ++ "/esdb_rs/target/release") : PkgDesc.extraLibDirs libraryBuildInfo
                          }
                    }
            }
      }

{-# LANGUAGE MultiWayIf    #-}
{-# LANGUAGE UnicodeSyntax #-}

import Hake

main ∷ IO ()
main = hake $ do

  "clean | clean the project" ∫
    cargo ["clean"] >> removeDirIfExists targetPath

  "update | update dependencies" ∫ cargo ["update"]

  buncherExecutable ♯
    cargo <| "build" : buildFlags

  "install | install to system" ◉ [buncherExecutable] ∰
    cargo <| "install" : buildFlags

  "test | build and test" ◉ [buncherExecutable] ∰ do
    cargo ["test"]
    cargo ["clippy"]
    rawSystem buncherExecutable ["--version"]
      >>= checkExitCode

 where
  targetPath ∷ FilePath
  targetPath = "target"

  buildPath ∷ FilePath
  buildPath = targetPath </> "release"

  buildFlags ∷ [String]
  buildFlags = [ "--release" ]

  buncherExecutable ∷ FilePath
  buncherExecutable =
    {- HLINT ignore "Redundant multi-way if" -}
    if | os ∈ ["win32", "mingw32", "cygwin32"] → buildPath </> "buncher.exe"
       | otherwise → buildPath </> "buncher"

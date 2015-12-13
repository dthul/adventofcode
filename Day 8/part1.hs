countinmem ('\\':'\\':xs) = 1 + countinmem xs
countinmem ('\\':'"':xs) = 1 + countinmem xs
countinmem ('\\':'x':_:_:xs) = 1 + countinmem xs
countinmem (_:xs) = 1 + countinmem xs
countinmem [] = 0

main = do
  s <- readFile "input.txt"
  let numinmem  = sum . map (subtract 2) . map countinmem . lines $ s
  let numincode = sum . map length . lines $ s
  print $ numincode - numinmem

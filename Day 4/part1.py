import hashlib

input = 'iwrupvqb'

n = 1
while True:
    hash = hashlib.md5('{}{}'.format(input, n).encode('ascii')).hexdigest()
    if hash.startswith('00000'):
        print('Santa can mine an AdventCoin with the number', n)
        print('Hash:', hash)
        break
    n += 1

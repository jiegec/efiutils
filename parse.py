import sys

with open(sys.argv[1], 'rb') as f:
    content = f.read().decode('utf-16')
    parts = content.split('&')
    for part in parts:
        lhs, rhs = part.split('=')
        if lhs == 'NAME':
            rhs = bytes.fromhex(rhs).decode('utf-16be')
            print(f'{lhs}={rhs}')
        else:
            print(part)

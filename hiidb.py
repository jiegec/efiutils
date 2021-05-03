# dump hiidb to binary form
# use https://github.com/LongSoft/Universal-IFR-Extractor to convert to text

import sys
import struct

with open(sys.argv[1], 'rb') as f:
    content = f.read().decode('utf-16')
    for line in content.split('\n'):
        line = line.strip()
        data = bytes.fromhex(line)
        if len(data) == 0:
            continue

        sys.stdout.buffer.write(data)
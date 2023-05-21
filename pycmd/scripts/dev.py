# @file:	dev.py
# @author:	Jacob Xie
# @date:	2023/04/07 18:14:16 Friday
# @brief:

import time

with open("./dev.csv") as f:
    count = 0
    while 1:
        line = f.readline()

        if not line:
            break
        print(f"Line{count}: {line.strip()}")
        count += 1

        time.sleep(1)

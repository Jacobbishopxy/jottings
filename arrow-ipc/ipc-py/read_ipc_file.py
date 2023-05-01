# @file:	dev.py
# @author:	Jacob Xie
# @date:	2023/04/28 10:22:33 Friday
# @brief:

import pyarrow.ipc as ipc

if __name__ == "__main__":
    """
    cd project root
    python ./arrow-ipc/ipc-py/dev.py
    """

    reader = ipc.open_file("dev.ipc")

    f = reader.read_all()

    print(f)

# @file:	dev.py
# @author:	Jacob Xie
# @date:	2023/04/19 17:38:02 Wednesday
# @brief:

import numpy as np
import pandas as pd
import time
import datetime as dt

# import qi.common_tools.np_data_loader as ndl

"""
pip install qi.common-tools -i http://10.144.64.195:8071/repository/jasperpypi-group/simple --trusted-host 10.144.64.195
"""


with open("./log/dev.log", "a") as myfile:
    while 1:
        t = dt.datetime.now()
        myfile.write(f"{t}\n")
        # myfile.flush()

        print("wrote: ", t)
        time.sleep(int(np.random.uniform(1, 10)))
        # ndl.df_to_cube

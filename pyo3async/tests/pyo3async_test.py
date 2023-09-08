# @file:	pyo3async_test.py
# @author:	Jacob Xie
# @date:	2023/09/08 23:53:40 Friday
# @brief:

import logging
import asyncio

from pyo3async import rust_sleep, rust_log

print("pyo3async test case")

FORMAT = "%(levelname)s %(name)s %(asctime)-15s %(filename)s:%(lineno)d %(message)s"
logging.basicConfig(
    level=logging.INFO,
    format=FORMAT,
    handlers=[
        #
        logging.FileHandler("debug.log"),
        logging.StreamHandler(),
    ],
)

rust_log()


async def py_sleep():
    # must be wrapped in a py async env
    await rust_sleep()


asyncio.run(py_sleep())

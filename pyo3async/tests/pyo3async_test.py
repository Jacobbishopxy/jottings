# @file:	pyo3async_test.py
# @author:	Jacob Xie
# @date:	2023/09/08 23:53:40 Friday
# @brief:

import logging
import asyncio

from pyo3async import rust_sleep, rust_log, PA, rust_sleep_print

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


pa = PA(1, "Jacob", ["a", "b", "c"])

print(pa.to_json())

print(pa.key)
print(pa.name)
print(pa.props)

pa.name = "JacobX"
print(pa.name)

pa.props = pa.props + ["tada"]
print(pa.props)


async def py_sleep_print(secs: int, pa: PA) -> PA:
    return await rust_sleep_print(secs, pa)


async_pa = asyncio.run(py_sleep_print(5, pa))

print(async_pa.to_json())

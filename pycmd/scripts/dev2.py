# @file:	dev2.py
# @author:	Jacob Xie
# @date:	2023/05/21 23:58:00 Sunday
# @brief:


import pyarrow as pa

days = pa.array([1, 12, 17, 23, 28], type=pa.int8())

months = pa.array([1, 3, 5, 7, 1], type=pa.int8())

years = pa.array([1990, 2000, 1995, 2000, 1995], type=pa.int16())

birthdays_table = pa.table([days, months, years], names=["days", "months", "years"])


print(birthdays_table)

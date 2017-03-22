#!/usr/bin/python
def countBits(num):
    """
    :type num: int
    :rtype: List[int]
    """
    ret=[None] * (num + 1)
    n=1
    for i in range(0, num + 1):
        if i == 0:
            ret[i] = 0
        elif i == n:
            ret[i] = 1
            n = n * 2
        else:
            ret[i] = ret[n / 2] + ret[i - n / 2]
    return ret

def numberOfArithmeticSlices(A):
    """
    :type A: List[int]
    :rtype: int
    """
    n = len(A)
    if n < 3:
        return 0

    sum = 0
    silcecLen = 2
    diff = A[1] - A[0]
    for i in range(2, n):
        newDiff = A[i] - A[i - 1]
        if diff == newDiff:
            silcecLen += 1
        else:
            if silcecLen >= 3:
                sum += silcecLen * (silcecLen - 3) / 2 + 1
            silcecLen = 2
            diff = newDiff
            
    if silcecLen >= 3:
        sum += silcecLen * (silcecLen - 3) / 2 + 1
    return sum

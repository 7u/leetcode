def findPoisonedDuration(timeSeries, duration):
    """
    :type timeSeries: List[int]
    :type duration: int
    :rtype: int
    """

    n = len(timeSeries)
    if n == 0:
        return 0
    
    head = 0
    totalTime = 0
    for i in range(1, n):
        d = timeSeries[i] - timeSeries[i - 1]
        if d > duration:
            totalTime += timeSeries[i - 1] - timeSeries[head] + duration
            head = i
            
    totalTime += timeSeries[n - 1] - timeSeries[head] + duration
    return totalTime

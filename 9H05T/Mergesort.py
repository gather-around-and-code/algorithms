import sys


def merge(A):
    if len(A) == 1:
        return A
    
    m = (len(A)+1) // 2
    p = merge(A[:m])
    q = merge(A[m:])

    
    return

if __name__ == '__main__':
    input = sys.stdin.readline
    A = list(map(int, input().split()))
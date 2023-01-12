import sys

# 자바스크립트의 readFileSync("/dev/stdin")와 유사한 sys.stdin을 사용하여 접근!
input_str = sys.stdin.read().strip()
words = input_str.split(' ')

count = 0
# 반복문을 사용하고 싶은 경우 
# for var in range(start, stop, step) 또는 for var in list 사용
for i in range(len(words)):
    if words[i] != '':
        count += 1
        # 파이썬에서 증가(++)는 +=1 

print(count)

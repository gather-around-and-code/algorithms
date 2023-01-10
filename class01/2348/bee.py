# 반복문을 이용해
# 직각 삼각형 만들기
def print_right_triangle(size): # (문법) 파이썬은 snake_case로 작성
    for i in range(1, size+1):
        print('*' * i)

print_right_triangle(5)
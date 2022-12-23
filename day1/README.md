# Two Sum (63 times): 

## 문제

정수 배열 nums와 정수 target이 주어졌을 때, target을 만들기 위한 두 개의 숫자의 인덱스를 반환하십시오. 각 입력은 정확히 한 개의 솔루션을 가지고 있다고 가정하고, 같은 요소를 두 번 이상 사용하지 않는다고 가정할 수 있습니다. 어떤 순서로든 정답을 반환할 수 있습니다.

## 예제

```bash
Input: nums = [2,7,11,15], target = 9
Output: [0,1]
Explanation: Because nums[0] + nums[1] == 9, we return [0, 1].
```

```bash
Input: nums = [3,2,4], target = 6
Output: [1,2]
```

```bash
Input: nums = [3,3], target = 6
Output: [0,1]
```


## 제약조건:

- `2 <= nums.length <= 104`
- `-109 <= nums[i] <= 109`
- `-109 <= target <= 109`
- `Only one valid answer exists.`

## 출처 : 
- [LeetCode](https://leetcode.com/)

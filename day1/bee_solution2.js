/**
 * @param {number[]} nums
 * @param {number} target
 * @return {number[]}
 */

const twoSum = (nums, target) => {
    const map = {}
    for (let i = 0; i < nums.length; i += 1) {
        const current = nums[i]
        const complement = target - current
        if (complement in map) {
            return [map[complement], i]
        }
        map[current] = i
    }
    return false
}
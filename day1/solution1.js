/**
 * @param {number[]} nums
 * @param {number} target
 * @return {number[]}
 */
 
const twoSum = (nums, target) => {
    const map = new Map()
    let result = []
    nums.map((v, i) => {
        const complement = target - v
        map.has(complement) ? result = [map.get(complement), i] : map.set(v, i)
    })
    return result
}
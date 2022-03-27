class Solution:
    def longestCommonPrefix(self, strs: List[str]) -> str:
        for i in range(1, len(strs[0]) + 1):
            for j in range(1, len(strs)):
                comparedString = strs[0][0:i]
                print(comparedString, strs[j][0:i], comparedString == strs[j][0:i])
                if comparedString != strs[j][0:i]: return comparedString[:-1]
        return strs[0]
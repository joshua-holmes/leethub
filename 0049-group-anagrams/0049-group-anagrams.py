class Solution:
    def groupAnagrams(self, strs: List[str]) -> List[List[str]]:
        map_of_words = {}
        for word in strs:
            key = "".join(sorted(word))
            inner_list = map_of_words.setdefault(key, [])
            inner_list.append(word)
        return list(map_of_words.values())
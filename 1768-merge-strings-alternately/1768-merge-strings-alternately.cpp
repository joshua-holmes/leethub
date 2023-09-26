class Solution {
public:
    string mergeAlternately(string word1, string word2) {
        string answer;
        int longestWord = std::max(word1.size(), word2.size());
        
        for (int i = 0; i < longestWord; i++) {
            if (i < word1.size())
                answer.push_back(word1[i]);
            if (i < word2.size())
                answer.push_back(word2[i]);
        }
        
        return answer;
    }
};
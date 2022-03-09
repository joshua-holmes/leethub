/**
 * @param {string} s
 * @return {boolean}
 */
var isValid = function(s) {
  const stack = [];
  const open = ['(', '{', '['];
  const close = [')', '}', ']'];
  // Loop through each item in the string
  for (let i = 0; i < s.length; i++) {
      // If item is opening parenthesis, add to stack
      if (open.find(p => p === s[i])) {
          stack.push(open.indexOf(s[i]))
      // If item is closing parenthesis, subtract from stack if parenthesis index matches
      } else if (close.find(p => p === s[i])) {
          if (stack[stack.length - 1] === close.indexOf(s[i])) {
              stack.pop();
          } else {
              return false;
          }
      }
  }
  return !stack.length;
};
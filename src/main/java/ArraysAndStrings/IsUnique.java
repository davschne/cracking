package cracking.arraysAndStrings;

import java.util.HashMap;
import java.util.Scanner;

class IsUnique {
  public static boolean isUnique(String string) {
    // create hash table
    // capacity based on char type
    // value of true means character is present in the string
    HashMap<Character, Boolean> hash = new HashMap<>(256);
    for (int i = 0; i < string.length(); i++) {
      char c = string.charAt(i);
      if (hash.containsKey(c)) {
        return false;
      }
      else {
        hash.put(c, true);
      }
    }
    return true;
  }

  public static void main(String[] args) {
    Scanner scanner = new Scanner(System.in);
    String input;
    while ((input = scanner.nextLine()).length() > 0) {
      // String input = scanner.next();
      System.out.println(IsUnique.isUnique(input) ? "true" : "false");
    }
  }
}

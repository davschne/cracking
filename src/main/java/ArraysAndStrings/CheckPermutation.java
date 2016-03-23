package cracking.arraysAndStrings;

import java.util.HashMap;
import java.util.Scanner;

public class CheckPermutation {
  public static boolean checkPermutation(String a, String b) {
    HashMap<Character, Integer> hash = new HashMap<>();
    // iterate String a, construct hash of characters:
    //   key : char , value : number of entries
    for (int i = 0; i < a.length(); i++) {
      char c  = a.charAt(i);
      int num = hash.containsKey(c) ? hash.get(c) : 0;
      hash.put(c, num + 1);
    }
    // iterate String b, deconstruct hash
    for (int i = 0; i < b.length(); i++) {
      char c = b.charAt(i);
      if (!hash.containsKey(c)) return false; // The character wasn't in a.
      int num = hash.get(c);
      // Remove key if it's the last such character.
      if (num == 1) hash.remove(c);
      // Otherwise decrement the number of such characters remaining.
      else hash.put(c, num - 1);
    }
    // If hash is now empty, both Strings contain the same characters.
    return hash.isEmpty();
  }

  // interactive command-line program
  public static void main(String[] args) {
    Scanner scanner = new Scanner(System.in);
    while (true) {
      String a = scanner.next();
      String b = scanner.next();
      System.out.println(CheckPermutation.checkPermutation(a, b));
    }
  }
}

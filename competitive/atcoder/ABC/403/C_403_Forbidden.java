import java.util.*;

public class Main {
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);
        int n = sc.nextInt();
        int m = sc.nextInt();
        int q = sc.nextInt();

        boolean[] allAuthority = new boolean[n + 1];
        Map<Integer, Set<Integer>> userPages = new HashMap<>();

        for (int i = 0; i < q; i++) {
            int type = sc.nextInt();
            int user = sc.nextInt();

            if (type == 1) {
                int contentPage = sc.nextInt();
                userPages.computeIfAbsent(user, k -> new HashSet<>()).add(contentPage);
            } else if (type == 2) {
                allAuthority[user] = true;
            } else {
                int contentPage = sc.nextInt();
                if (allAuthority[user] ||
                    (userPages.containsKey(user) && userPages.get(user).contains(contentPage))) {
                    System.out.println("Yes");
                } else {
                    System.out.println("No");
                }
            }
        }
    }
}

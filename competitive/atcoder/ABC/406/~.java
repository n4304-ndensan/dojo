import java.util.*;

public class Main {
    static class Pair {
        char first;
        long second;
        Pair(char f, long s) {
            this.first = f;
            this.second = s;
        }
    }

    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);
        int n = sc.nextInt();
        int[] p = new int[n];
        for (int i = 0; i < n; i++) {
            p[i] = sc.nextInt();
        }

        List<Pair> v = new ArrayList<>();

        for (int i = 0; i < n - 1; i++) {
            if (p[i] < p[i + 1]) {
                if (v.isEmpty() || v.get(v.size() - 1).first == '>') {
                    v.add(new Pair('<', 1));
                } else {
                    v.get(v.size() - 1).second++;
                }
            } else {
                if (v.isEmpty() || v.get(v.size() - 1).first == '<') {
                    v.add(new Pair('>', 1));
                } else {
                    v.get(v.size() - 1).second++;
                }
            }
        }

        int sz = v.size();
        long ans = 0;
        for (int i = 1; i < sz - 1; i++) {
            if (v.get(i).first == '>') {
                ans += v.get(i - 1).second * v.get(i + 1).second;
            }
        }

        System.out.println(ans);
    }
}

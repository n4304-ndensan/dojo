import java.util.*;

public class Main {
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);
        int n = sc.nextInt();
        int k = sc.nextInt();

        int mod = 1_000_000_000;

        int[] a = new int[Math.max(k, n + 1)];
        int[] sumList = new int[a.length + 1];

        for (int i = 0; i < k; i++) {
            a[i] = 1;
            sumList[i + 1] = (sumList[i] + a[i]) % mod;
        }

        for (int i = k; i <= n; i++) {
            a[i] = (sumList[i] - sumList[i - k] + mod) % mod;
            sumList[i + 1] = (sumList[i] + a[i]) % mod;
        }

        System.out.println(a[n]);
    }
}

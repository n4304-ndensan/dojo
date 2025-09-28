import java.util.*;

public class Main {
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);
        Integer n = sc.nextInt();

        long ans = 0;
        for (Integer i = 1; i < n + 1; i++) {
            ans += Math.pow(-1, i) * Math.pow(i, 3);
        }

        System.out.println(ans);
    }
}

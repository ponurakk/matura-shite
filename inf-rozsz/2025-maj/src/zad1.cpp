#include <cstdio>

long przestaw(long n, int *count) {
  *count += 1;
  long r = n % 100;
  long a = r / 10;
  long b = r % 10;
  n = n / 100;

  long w = 0;
  if (n > 0) {
    w = a + 10 * b + 100 * przestaw(n, count);
  } else {
    if (a > 0) {
      w = a + 10 * b;
    } else {
      w = b;
    }
  }

  return w;
}

long przestaw2(long number) {
  long result = 0;
  int i = 0;

  while (number > 0) {
    long tens = number % 100;
    long numOnes = tens % 10;
    long numTens = tens / 10;

    long power = 1;
    for (long j = 0; j < i; j++)
      power = power * 10;

    if (number > 9) {
      result += (numOnes * 10 + numTens) * power;
    } else {
      result += numOnes * power;
    }

    number = number / 100;

    i += 2;
  }

  return result;
}

int main() {
  long int1 = 316498;
  long int2 = 43657688;
  long int3 = 154005710;
  long int4 = 998877665544321;

  int count1 = 0;
  int count2 = 0;
  int count3 = 0;
  int count4 = 0;

  printf("wtf1: %ld -> %ld\n", int1, przestaw(int1, &count1));
  printf("wtf2: %ld -> %ld\n", int2, przestaw(int2, &count2));
  printf("wtf3: %ld -> %ld\n", int3, przestaw(int3, &count3));
  printf("wtf4: %ld -> %ld\n", int4, przestaw(int4, &count4));

  printf("count1: %d\n", count1);
  printf("count2: %d\n", count2);
  printf("count3: %d\n", count3);
  printf("count4: %d\n", count4);

  printf("\n1.2: FPPF\n\n");

  printf("1.3: %ld -> %ld\n", int1, przestaw2(int1));
  printf("1.3: %ld -> %ld\n", int2, przestaw2(int2));
  printf("1.3: %ld -> %ld\n", int3, przestaw2(int3));
  printf("1.3: %ld -> %ld\n", int4, przestaw2(int4));

  return 0;
}

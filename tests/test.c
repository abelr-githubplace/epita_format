#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

typedef struct {
    int year;
    int month;
    int day;
    int hour;
    int minute;
    int second;
    int timezone_offset; // in minutes
} DateTime;

void set_date(DateTime *dt, int year, int month, int day) {
    dt->year = year;
    dt->month = month;
    dt->day = day;
}

void set_time(DateTime *dt, int hour, int minute, int second) {
    dt->hour = hour;
    dt->minute = minute;
    dt->second = second;
}

void set_timezone(DateTime *dt, int offset) {
    dt->timezone_offset = offset;
}

void get_date(const DateTime *dt, int *year, int *month, int *day) {
    *year = dt->year;
    *month = dt->month;
    *day = dt->day;
}

void get_time(const DateTime *dt, int *hour, int *minute, int *second) {
    *hour = dt->hour;
    *minute = dt->minute;
    *second = dt->second;
}

void get_timezone(const DateTime *dt, int *offset) {
    *offset = dt->timezone_offset;
}

void convert_timezone(DateTime *dt, int new_offset) {
    int offset_difference = new_offset - dt->timezone_offset;
    dt->timezone_offset = new_offset;

    int total_minutes = (dt->hour * 60 + dt->minute) + offset_difference;
    dt->hour = (total_minutes / 60) % 24;
    dt->minute = total_minutes % 60;
}

void print_datetime(const DateTime *dt) {
    printf("Date: %04d-%02d-%02d\n", dt->year, dt->month, dt->day);
    printf("Time: %02d:%02d:%02d (UTC%+d)\n", dt->hour, dt->minute, dt->second, dt->timezone_offset / 60);
}

int is_leap_year(int year) {
    return (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0);
}

int days_in_month(int year, int month) {
    if (month == 2) {
        return is_leap_year(year) ? 29 : 28;
    }
    return (month == 4 || month == 6 || month == 9 || month == 11) ? 30 : 31;
}

int validate_date(int year, int month, int day) {
    if (month < 1 || month > 12 || day < 1) return 0;
    return day <= days_in_month(year, month);
}

int validate_time(int hour, int minute, int second) {
    return (hour >= 0 && hour < 24) && (minute >= 0 && minute < 60) && (second >= 0 && second < 60);
}

int validate_datetime(const DateTime *dt) {
    return validate_date(dt->year, dt->month, dt->day) && validate_time(dt->hour, dt->minute, dt->second);
}

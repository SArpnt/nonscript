// this is supposed to be the expected output
#include <stdio.h>
/*
struct String {
	length:
}*/ // TODO: string should be length based and utf-16

char* BoolToString(const char b) {
	return b ? "true" : "false";
}
char* NumberToString(const double n) {
	;// TODO
}

struct _Tuple {};
char* _TupleToString (const struct _Tuple t) {
	return "()";
}
struct _Tuple_Number_Boolean_String {
	const double _1;
	const char _2;
	const char* _3;
};
char* _Tuple_Number_Boolean_StringToString (const struct _Tuple_Number_Boolean_String t) {
	const char* s1 = numberToString(t._1);
	const char* s2 = boolToString(t._2);
	size_t charCount = snprintf(NULL, 0, "(%s, %s, %s)", s1, s2, t._3); // TODO: make more efficient
	const char* buffer = malloc(charCount);
	sprintf(buffer, "(%s, %s, %s)", s1, s2, t._3);
	return buffer;
}

struct _Table_array_String { // 0 and up integers, no gaps
	char** items;
	size_t length;
	size_t capacity;
};
struct _Table_array_String* _newTable_array_String(const size_t capacity) {
	struct _Table_array_String t = {
		malloc(capacity * sizeof(char*)),
		0,
		capacity,
	};
	return &t;
}
void _freeTable_array_String(const struct _Table_array_String* t) {
	for (size_t i = 0; i < t->items; i++) // TODO: make sure there isn't anything unneccecary here or anything that leaks memory
		free(t->items[i]);
	free(t->items);
	free(t);
}
char* _Table_array_StringToString(const struct _Table_array_String* t) {
	size_t capacity = 8; // TODO: better starting value
	char* buffer = malloc(capacity);
	size_t written = snprintf(buffer, capacity, "%zx{", t);
	for (size_t i = 0; i < t->length; i++) {
		written += snprintf(buffer + written, capacity - written, "%zu: \"%s\"", i, t->items[i]); // TODO: handle when capacity too small
	}
	buffer[written + 1] = '}';
	return buffer;
}

struct _TableItem_String_Tuple_Number_Boolean_String {
	const char* key;
	struct _Tuple_Number_Boolean_String value;
};
struct _Table_String_Tuple_Number_Boolean_String {
	struct _TableItem_String_Tuple_Number_Boolean_String* items;
	size_t length;
	size_t capacity;
};
struct _Table_String_Tuple_Number_Boolean_String* _newTable_String_Tuple_Number_Boolean_String(const size_t capacity) {
	struct _Table_String_Tuple_Number_Boolean_String t = {
		malloc(capacity * sizeof(struct _TableItem_String_Tuple_Number_Boolean_String)),
		0,
		capacity,
	};
	return &t;
}
void _freeTable_String_Tuple_Number_Boolean_String(struct _Table_String_Tuple_Number_Boolean_String* t) {
	for (size_t i = 0; i < t->items; i++) { // TODO: make sure there isn't anything unneccecary here or anything that leaks memory
		free(t->items[i].key);
		free(t->items[i].value);
	}
	free(t->items);
	free(t);
}
char* _Table_String_Tuple_Number_Boolean_StringToString(struct _Table_String_Tuple_Number_Boolean_String* t) {
	char* buffer = malloc(64);
	for (size_t i = 0; i < t->length; i++) {
		t->items; // TODO
	}
}

int main() {
	printf("%s %s %s\n", "Boolean:", boolToString(1), boolToString(0));
	printf("%s %s %s %s %s %s %s %s %s %s\n", "Number:", "3", "2.4", "7.8", "-0.4", "800000", "600", "7e-4", "1e+100", "-0"); // numbers convert to string like in js
	printf("%s %s %s %s\n", "String:", "stuff", "quoted \"stuff\"", "a\nb");
	{
		char* tnbs = _Tuple_Number_Boolean_StringToString((struct _Tuple_Number_Boolean_String){5.0, 1, "bar"});
		printf("%s %s %s %s\n", "Tuple:", _TupleToString((struct _Tuple){}), "one", tnbs);
		free(tnbs);
	}
	{
		struct _Table* t1 = _newTable(0); // TODO: what type is an empty table?
		struct _Table_array_String* t2 = _newTable_array_String(8); // TODO: better starting value
		struct _Table_array_String* t3 = _newTable_array_String(8); // TODO: better starting value
		struct _Table_String_Tuple_Number_Boolean_String* t4 = _newTable_String_String(8); // TODO: better starting value
		t2->length = 3;
		t2->items[0] = "a";
		t2->items[1] = "b";
		t2->items[2] = "c";
		t3->length = 3;
		t3->items[0] = "a";
		t3->items[1] = "b";
		t3->items[2] = "c";
		t4->length = 3;
		t4->items[1] = (struct _TableItem_String_Tuple_Number_Boolean_String){"bar", (struct _Tuple_Number_Boolean_String){1, 1, "b"}};
		t4->items[0] = (struct _TableItem_String_Tuple_Number_Boolean_String){"foo", (struct _Tuple_Number_Boolean_String){2, 0, "a"}};
		t4->items[2] = (struct _TableItem_String_Tuple_Number_Boolean_String){"2", (struct _Tuple_Number_Boolean_String){3, 0, "c"}};
		char* ts1 = _TableToString(t1);
		char* ts2 = _Table_array_StringToString(t2);
		char* ts3 = _Table_array_StringToString(t3);
		char* ts4 = _Table_String_Tuple_Number_Boolean_StringToString(t4);
		printf("%s %s %s %s %s\n", "Table:", ts1, ts2, ts3, ts4);
		free(ts1);
		free(ts2);
		free(ts3);
		free(ts4);
		//free(t1.items); // TODO
		_freeTable_array_String(&t2);
		_freeTable_array_String(&t3);
		_freeTable_String_Tuple_Number_Boolean_String(&t4);
	}
	printf("%s %s %s %s\n", "Function:", "(a, b) => a + b", "{a,b}=>a+b", "args\t=>(args[a] +args[b])");
}
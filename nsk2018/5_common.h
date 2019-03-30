#pragma once

#include <vector>
#include <iostream>

using namespace std;
typedef int64_t bint;

struct fraction 
{
	bint a, b;

	fraction();
	explicit fraction(bint a1, bint b1 = 1);

	fraction& optimize(void);
	fraction& operator+=(const fraction& f);
	fraction& operator*=(const bint& f);
	fraction& operator*=(const fraction& f);
	fraction& operator/=(const bint& f);
	fraction& operator/=(const fraction& f);
};

class prob_line : public vector<fraction> {
public:
	prob_line(size_t n, const fraction& f);

	prob_line& operator+=(const prob_line& line);
	prob_line& operator*=(const bint& f);
	prob_line& operator*=(const fraction& f);
	prob_line& operator/=(const bint& f);
	prob_line& operator/=(const fraction& f);
};

typedef vector<prob_line> prob_table;

ostream& operator<<(ostream& out, const fraction& a);
ostream& operator<<(ostream& out, const prob_line& line);
ostream& operator<<(ostream& out, const prob_table& table);

//=============================================================================
//=============================================================================
//=============================================================================

//-----------------------------------------------------------------------------
inline fraction::fraction() : a(0), b(1) {}

//-----------------------------------------------------------------------------
inline fraction::fraction(bint a1, bint b1) : a(a1), b(b1) {
	if (a == 0 && b == 0) 
		b = 1;
}

//-----------------------------------------------------------------------------
inline fraction& fraction::optimize(void) {
	if (b != 1) {
		bint g = gcd(a, b);
		if (g != 0) {
			a /= g;
			b /= g;
		}
	}
	return *this;
}

//-----------------------------------------------------------------------------
inline fraction& fraction::operator+=(const fraction& f) {
	bint tempa = a, tempb = b;
	a = tempa * f.b + f.a * tempb;
	b = tempb * f.b;
	return optimize();
}

//-----------------------------------------------------------------------------
inline fraction& fraction::operator*=(const bint& f) {
	a *= f;
	return optimize();
}

//-----------------------------------------------------------------------------
inline fraction& fraction::operator*=(const fraction& f) {
	a *= f.a;
	b *= f.b;
	return optimize();
}

//-----------------------------------------------------------------------------
inline fraction& fraction::operator/=(const bint& f) {
	b *= f;
	return optimize();
}

//-----------------------------------------------------------------------------
inline fraction& fraction::operator/=(const fraction& f) {
	a *= f.b;
	b *= f.a;
	return optimize();
}

//-----------------------------------------------------------------------------
//-----------------------------------------------------------------------------
//-----------------------------------------------------------------------------

//-----------------------------------------------------------------------------
inline prob_line::prob_line(size_t n, const fraction& f) : vector<fraction>(n, f) {}

//-----------------------------------------------------------------------------
inline prob_line& prob_line::operator+=(const prob_line& line) {
	for (int i = 0; i < line.size(); i++)
		(*this)[i] += line[i];
	return *this;
}

//-----------------------------------------------------------------------------
inline prob_line& prob_line::operator*=(const bint& f) {
	for (auto& i : *this)
		i *= f;
	return *this;
}

//-----------------------------------------------------------------------------
inline prob_line& prob_line::operator*=(const fraction& f) {
	for (auto& i : *this)
		i *= f;
	return *this;
}

//-----------------------------------------------------------------------------
inline prob_line& prob_line::operator/=(const bint& f) {
	for (auto& i : *this)
		i /= f;
	return *this;
}

//-----------------------------------------------------------------------------
inline prob_line& prob_line::operator/=(const fraction& f) {
	for (auto& i : *this)
		i /= f;
	return *this;
}

//-----------------------------------------------------------------------------
//-----------------------------------------------------------------------------
//-----------------------------------------------------------------------------

//-----------------------------------------------------------------------------
inline ostream& operator<<(ostream& out, const fraction& a) {
	if (a.b == 1) {
		if (a.a == 0)
			out << "_";
		else
			out << a.a;
	} else
		out << (to_string(a.a) + "/" + to_string(a.b));
	return out;
}

//-----------------------------------------------------------------------------
inline ostream& operator<<(ostream& out, const prob_line& line) {
	for (auto& i : line)
		out << setw(6) << i;
	return out;
}

//-----------------------------------------------------------------------------
inline ostream& operator<<(ostream& out, const prob_table& table) {
	for (auto& i : table)
		out << i << endl;
	return out;
}
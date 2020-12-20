#include <algorithm>
#include <cmath>
#include <exception>
#include <fstream>
#include <functional>
#include <iomanip>
#include <iostream>
#include <map>
#include <memory>
#include <queue>
#include <random>
#include <set>
#include <sstream>
#include <string>
#include <tuple>
#include <vector>

//#include <twg/twg.h>
//#include <twg/image/image_drawing.h>
//#define USE_TWG

//#include <boost/multiprecision/cpp_bin_float.hpp>
//typedef boost::multiprecision::cpp_bin_float_quad realreal;
typedef double realreal;

#undef min
#undef max

//-----------------------------------------------------------------------------
//-----------------------------------------------------------------------------
//-----------------------------------------------------------------------------
// Мой код Мой код Мой код Мой код Мой код Мой код Мой код Мой код Мой код Мой 
using namespace std;

//-----------------------------------------------------------------------------
const realreal mypi = 3.14159265358979323846264338327950288419716939937510582097494459230781640628620899862803482534211706798214808651328230664709384460955058223172535940812848111745028410270193852110555964462294895493038196442881097566593344612847564823378678316527120190914;

struct point { realreal x, y; };
struct line { point a, b; };

struct input 
{
	int n;
	vector<point> p;
	point min;
	point max;
} in;
const realreal line_around = 0.0000001;
const realreal delete_around = 0.00000001;
const realreal zero_around = 0.00001;
//-----------------------------------------------------------------------------
//-----------------------------------------------------------------------------
//-----------------------------------------------------------------------------

//-----------------------------------------------------------------------------
// /\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\
// /\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\
// /\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\
// КОД ДЛЯ ОПРЕДЕЛЕНИЯ, НАХОДИТСЯ ЛИ ТОЧКА ВНУТРИ ПОЛИГОНА
// Нагло скопирован с https://github.com/pglauner/point_in_polygon

//-----------------------------------------------------------------------------
struct Line {
	point p1;
	point p2;
};

//-----------------------------------------------------------------------------
static int ccw(const point& p0, const point& p1, const point& p2) {
	double dx1 = double(p1.x - p0.x);
	double dy1 = double(p1.y - p0.y);
	double dx2 = double(p2.x - p0.x);
	double dy2 = double(p2.y - p0.y);
	double dx1dy2 = dx1 * dy2;
	double dx2dy1 = dx2 * dy1;

	if (dx1dy2 > dx2dy1) {
		return 1;
	} else if (dx1dy2 < dx2dy1) {
		return -1;
	}
	else {
		if (dx1 * dx2 < 0 || dy1 * dy2 < 0) {
			return -1;
		} else if (dx1 * dx1 + dy1 * dy1 >= dx2 * dx2 + dy2 * dy2) {
			return 0;
		} else {
			return 1;
		}
	}
}

//-----------------------------------------------------------------------------
inline int intersect(const Line& line1, const Line& line2) {
	int ccw11 = ccw(line1.p1, line1.p2, line2.p1); if (ccw11 == 0) return 1;
	int ccw12 = ccw(line1.p1, line1.p2, line2.p2); if (ccw12 == 0) return 1;
	if (!(ccw11 * ccw12 < 0))
		return 0;
	int ccw21 = ccw(line2.p1, line2.p2, line1.p1); if (ccw21 == 0) return 1;
	int ccw22 = ccw(line2.p1, line2.p2, line1.p2); if (ccw22 == 0) return 1;
	return (ccw21 * ccw22 < 0) ? 1 : 0;
}

//-----------------------------------------------------------------------------
inline int getNextIndex(int n, int current) {
	return current == n - 1 ? 0 : current + 1;
}

//-----------------------------------------------------------------------------
bool pointInPolygon(point testPoint) {
	static std::vector<point> polygon;
	static bool isFirst = true;
	if (isFirst) {
		polygon.reserve(100);
		isFirst = false;
	}

	polygon = in.p;
	int n = polygon.size();
	Line xAxis;
	Line xAxisPositive;

	point startPoint;
	point endPoint;
	Line edge;
	Line testPointLine;

	int i;
	double startNodePosition;
	int count;
	int seenPoints;

	startPoint.x = 0;
	startPoint.y = 0;

	xAxis.p1.x = 0;
	xAxis.p1.y = 0;
	xAxis.p2.x = 0;
	xAxis.p2.y = 0;
	xAxisPositive.p1.x = 0;
	xAxisPositive.p1.y = 0;
	xAxisPositive.p2.x = 0;
	xAxisPositive.p2.y = 0;

	startNodePosition = -1;

	for (i = 0; i < n; i++) {
		if (testPoint.x == polygon[i].x && testPoint.y == polygon[i].y) {
			return 1;
		}

		polygon[i].x -= testPoint.x;
		polygon[i].y -= testPoint.y;

		if (polygon[i].y != 0) {
			startPoint.x = polygon[i].x;
			startPoint.y = polygon[i].y;
			startNodePosition = i;
		}

		if (polygon[i].x > xAxis.p2.x) {
			xAxis.p2.x = polygon[i].x;
			xAxisPositive.p2.x = polygon[i].x;
		}
		if (polygon[i].x < xAxis.p1.x) {
			xAxis.p1.x = polygon[i].x;
		}
	}

	testPoint.x = 0;
	testPoint.y = 0;
	testPointLine.p1 = testPoint;
	testPointLine.p2 = testPoint;

	for (i = 0; i < polygon.size(); i++) {
		edge.p1 = polygon[i];
		edge.p2 = polygon[getNextIndex(polygon.size(), i)];
		if (intersect(testPointLine, edge) == 1) {
			return 1;
		}
	}

	if (startNodePosition == -1) {
		return 0;
	}

	count = 0;
	seenPoints = 0;
	i = int(startNodePosition);
	while (seenPoints < n) {
		double savedX(polygon[getNextIndex(n, i)].x);
		int savedIndex = getNextIndex(n, i);
		do {
			i = getNextIndex(n, i);
			seenPoints++;
		} while (polygon[i].y == 0);
		endPoint.x = polygon[i].x;
		endPoint.y = polygon[i].y;
		if (startPoint.y * endPoint.y < 0) {
			edge.p1 = startPoint;
			edge.p2 = endPoint;
			if (savedIndex == i) {
				count += intersect(edge, xAxisPositive);
			}
			else if (savedX > 0) {
				count += intersect(edge, xAxis);
			}
		}
		startPoint = endPoint;
	}
	return count % 2;
}
// \/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/
// \/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/
// \/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/
//-----------------------------------------------------------------------------

//-----------------------------------------------------------------------------
// /\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\
// /\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\
// /\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\
// Код библиотеки для дифференциальной эволюции, просто скопирован отсюда:
// http://www.amichel.com/de/doc/html/

namespace de
{
inline double genrand( double min = 0, double max = 1 )
{
	//static boost::random::mt19937 gen;
	//boost::random::uniform_real_distribution<> dist( min, max );
	//boost::variate_generator<boost::random::mt19937&, boost::random::uniform_real_distribution< double > > value(gen, dist);
	//return value();
	static std::mt19937 gen;
	std::uniform_real_distribution<> dist( min, max );
	//boost::variate_generator<std::mt19937&, std::uniform_real_distribution< double > > value(gen, dist);
	//return value();
	return dist(gen);
}
inline int genintrand( double min, double max, bool upperexclusive = false )
{
	//assert( min < max );
	int ret = 0;
	do ret = std::round( genrand( min, max ) ); while( ret < min || ret > max || upperexclusive && ret == max );
	return ret;
}
}
namespace de
{
//typedef boost::recursive_mutex mutex;
//typedef boost::lock_guard< boost::recursive_mutex > lock;
}
namespace de
{
class myreal
{
private:
	double m_value;
public:
	myreal( double value )
	: m_value( value )
	{
	}
	myreal()
	: m_value( 0 )
	{
	}
	double operator=( double value )
	{
		m_value = value;
		return m_value;
	}
	operator double() const
	{
		return m_value;
	}
};
typedef std::vector< myreal > DVector;
typedef std::shared_ptr< DVector > DVectorPtr;
class exception : public std::exception
{
private:
	const std::string m_what;
public:
	virtual ~exception() throw()
	{
	}
	exception( const char* what )
	: m_what( what != 0 ? what : "" )
	{
	}
	virtual const char* what() const throw()
	{
		return m_what.c_str();
	}
};
}
namespace de
{
class constraints_exception : public exception
{
public:
	constraints_exception( const std::string& message )
	: exception( message.c_str() )
	{
	}
};
class constraint
{
public:
	virtual ~constraint(){}
	virtual double get_rand_value() = 0;
	virtual double get_rand_value( double value, double origin ) = 0;
	virtual double min() const = 0;
	virtual double max() const = 0;
	virtual double get_rand_value_in_zone( double origin, double zonePct ) const = 0;
	virtual double get_middle_point() = 0;
};
typedef std::shared_ptr< constraint > constraint_ptr;
class range_constraint : public constraint
{
private:
	double m_min;
	double m_max;
public:
	range_constraint( double min, double max )
	: m_min( min ), m_max( max )
	{
		//assert( min <= max );
	}
	double min() const { return m_min; }
	double max() const { return m_max; }
};
class real_constraint : public range_constraint
{
public:
	real_constraint( double min, double max )
	: range_constraint( min, max )
	{
		//assert( min <= max );
	}
	double get_rand_value()
	{
		return genrand( range_constraint::min(), range_constraint::max() );
	}
	double get_rand_value( double value, double origin )
	{
		double ret = value;
		while( ret < range_constraint::min() )
		{
			ret = range_constraint::min() + genrand() * ( origin - range_constraint::min() );
		}
		while( ret > range_constraint::max() )
		{
			ret = range_constraint::max() + genrand() * ( origin - range_constraint::max() );
		}
		return ret;
	}
	virtual double get_rand_value_in_zone( double origin, double zonePct ) const
	{
		if( origin > max() )
			throw constraints_exception( "origin coordinate > max" );
		if( origin < min() )
			throw constraints_exception( "origin coordinate < min" );
		if( zonePct > 100.0 )
			throw constraints_exception( "zonePct > 100%" );
		if( zonePct < 0 )
			throw constraints_exception( "zonePct < 0%" );
		if( zonePct == 0 )
			throw constraints_exception( "zonePct == 0%" );
		double zoneSize = ( max() - min() ) * zonePct / 100.0;
		double _min = std::max( min(), origin - zoneSize/2.0 );
		double _max = std::min( max(), origin + zoneSize/2.0 );
		return genrand( _min, _max );
	}
	virtual double get_middle_point()
	{
		return ( max() + min() )/2.0;
	}
};
class int_constraint : public range_constraint
{
public:
	int_constraint( double min, double max )
	: range_constraint( min, max )
	{
		//assert( min <= max );
	}
	double get_rand_value()
	{
		return genintrand( range_constraint::min(), range_constraint::max() );
	}
	double get_rand_value( double value, double origin )
	{
		double ret = std::round( value );
		while( ret < range_constraint::min() )
		{
			ret = range_constraint::min() + genrand() * ( origin - range_constraint::min() );
			ret = std::round( ret );
		}
		while( ret > range_constraint::max() )
		{
			ret = range_constraint::max() + genrand() * ( origin - range_constraint::max() );
			ret = std::round( ret );
		}
		return ret;
	}
	virtual double get_rand_value_in_zone( double origin, double zonePct ) const
	{
		if ( origin > max() )
			throw constraints_exception( "origin coordinate > max" );
		if ( origin < min() )
			throw constraints_exception( "origin coordinate < min" );
		if ( zonePct > 100.0 )
			throw constraints_exception( "zonePct > 100%" );
		if( zonePct < 0 )
			throw constraints_exception( "zonePct < 0%" );
		if( zonePct == 0  )
			throw constraints_exception( "zonePct == 0%" );
		double zoneSize = ( max() - min() ) * zonePct/100.0;
		double _min = std::max( min(), origin - zoneSize/2.0 );
		double _max = std::min( max(), origin + zoneSize/2.0 );
		double val = std::round( genrand( _min, _max ) );
		for ( ;val < _min || val > _max; val = std::round( genrand( _min, _max ) ) );
		return val;
	}
	virtual double get_middle_point()
	{
		return std::round( ( max() - min() )/2.0 );
	}
};
class set_constraint : public constraint
{
private:
	class unique : public std::unary_function < double, bool >
	{
	public:
		bool operator ()( double d ) const 
		{
			return m_unique.insert( d ).second;
		}
	public:
		double min() const
		{
			if( m_unique.size() > 0 )
				return *m_unique.begin();
			else
				throw constraints_exception( "could not get the min value of an empty set constraint" );
		}
		double max() const
		{
			if( m_unique.size() > 0 )
				return *m_unique.rbegin();
			else
				throw constraints_exception( "could not get the max value of an empty set constraint" );
		}
	private:
		mutable std::set< double > m_unique;
	};
private:
	unique m_unique;
	de::DVector m_values;
public:
	set_constraint( const de::DVector& values )
	{
	
		std::remove_copy_if (values.begin (), values.end (), std::back_inserter( m_values ), m_unique );
	}
	void add_value( de::myreal value )
	{
		if( m_unique( value ) )
			m_values.push_back( value );
	}
	virtual double get_rand_value()
	{
		de::DVector::size_type index( genintrand( 0, m_values.size() - 1 ) );
		return m_values[ index ];
	}
	virtual double get_rand_value(double value, double origin)
	{
		return get_rand_value();
	}
	double min() const
	{
		return m_unique.min();
	}
	double max() const
	{
		return m_unique.max();
	}
	virtual double get_rand_value_in_zone( double origin, double zonePct ) const
	{
		throw constraints_exception( "get_rand_value_in_zone only supported for range constraints" );
	}
	virtual double get_middle_point()
	{
		throw constraints_exception( "get_middle_point not supported by set constraint" );
	}
};
class boolean_constraint : public constraint
{
public:
	virtual double get_rand_value()
	{
		return genrand() < 0.5;
	}
	virtual double get_rand_value(double value, double origin)
	{
		return get_rand_value();
	}
	double min() const
	{
		return 0;
	}
	double max() const
	{
		return 1;
	}
	virtual double get_rand_value_in_zone( double origin, double zonePct ) const
	{
		throw constraints_exception( "get_rand_value_in_zone only supported for range constraints" );
	}
	virtual double get_middle_point()
	{
		throw constraints_exception( "get_middle_point not supported by bool constraint" );
	}
};
typedef std::vector< constraint_ptr > constraints_base;
class constraints : public constraints_base
{
private:
	//typedef boost::char_separator< char > separator;
	//typedef boost::tokenizer< separator > tokenizer;
public:
	constraints( size_t varCount, double defMin, double defMax )
	: constraints_base( varCount, std::make_shared< real_constraint >( defMin, defMax ) )
	{
	}
	constraints( const std::vector< std::string >& str, size_t var_count , double def_min, double def_max )
	: constraints_base( var_count, std::make_shared< real_constraint >( def_min, def_max ) )
	{
		// for( std::vector< std::string >::size_type i = 0; i < str.size(); ++i )
		// {
		// 	tokenizer tokens( str[ i ], separator( ";," ) );
		// 	std::string type;
		// 	double _min;
		// 	double _max;
		// 	size_t count( 0 );
		// 	for( tokenizer::const_iterator j = tokens.begin(); j != tokens.end(); ++j, ++count )
		// 	{
		// 		const std::string token( boost::trim_copy( *j ) );
		// 		try
		// 		{
		// 			switch( count )
		// 			{
		// 				case 0:
		// 					type = token;
		// 					break;
		// 				case 1:
		// 					_min = boost::lexical_cast< double >( token.c_str() );
		// 					break;
		// 				case 2:
		// 					_max = boost::lexical_cast< double >( token.c_str() );
		// 					break;
		// 				//default:
						
		// 					//throw constraints_exception( ( boost::format( "wrong variable format in \"%1%\" - too many fields" ) % str[ i ] ).str() );
		// 			}
		// 		}
		// 		catch( const boost::bad_lexical_cast& )
		// 		{
		// 			//throw constraints_exception( ( boost::format( "wrong floating point number format: %1%") % token ).str() );
		// 		}
		// 	}
		
		// 	if( count < 3 )
		// 		//throw constraints_exception( ( boost::format( "wrong variable format in \"%1%\" - too few fields" ) % str[ i ] ).str() );
		// 	if( i < var_count )
		// 		constraints_base::at( i ) = str_to_constraint( type, _min, _max );
		// 	else
		// 		constraints_base::push_back( str_to_constraint( type, _min, _max ) );
		// }
	}
	double get_rand_value( size_t index )
	{
		if( index < constraints_base::size() )
			return (*this)[ index ]->get_rand_value();
		//else
			//throw constraints_exception( ( boost::format( "invalid constraint index: %1%, higher than max number of constraints: %2%" ) % index % constraints_base::size() ).str() );
	}
	double get_rand_value( size_t index, double value, double origin )
	{
		if( index < constraints_base::size() )
			return (*this)[ index ]->get_rand_value( value, origin );
		//else
			//throw constraints_exception( ( boost::format( "invalid constraint index: %1%, higher than max number of constraints: %2%" ) % index % constraints_base::size() ).str() );
	}
	DVectorPtr get_square_zone_rand_values( const DVectorPtr origin, double sidePct ) const
	{
		//assert( origin );
		//assert( sidePct > 0 && sidePct <= 100 );
		if( origin->size() == constraints_base::size() )
		{
			DVectorPtr square( std::make_shared< DVector >( origin->size() ) );
			for( constraints_base::size_type n = 0; n < constraints_base::size(); ++n )
				(*square)[ n ] = (*this)[ n ]->get_rand_value_in_zone( (*origin)[ n ], sidePct );
			return square;
		}
		else
			throw constraints_exception( "The origin vector must have the same number of elements as there are constraints" );
	}
	DVectorPtr get_middle_point()
	{
		DVectorPtr r( std::make_shared< DVector >( constraints_base::size() ) );
		for( constraints_base::size_type n = 0; n < constraints_base::size(); ++n )
			(*r)[ n ] = (*this)[ n ]->get_middle_point();
		return r;
	}
	DVectorPtr get_rand_values() const
	{
		DVectorPtr r( std::make_shared< DVector >( constraints_base::size() ) );
		for( constraints_base::size_type n = 0; n < constraints_base::size(); ++n )
			(*r)[ n ] = (*this)[ n ]->get_rand_value();
		return r;
	}
private:
	constraint_ptr str_to_constraint( const std::string& type, double min, double max )
	{
		// if( boost::to_lower_copy( type ) == "real" )
		// 	return std::make_shared< real_constraint >( min, max );
		// else if( boost::to_lower_copy( type ) == "int" || boost::to_lower_copy( type ) == "integer" )
		// 	return std::make_shared< int_constraint >( min, max );
		// //else
		// 	//throw constraints_exception( ( boost::format( "invalid constraint type \"%1%\"" ) % type ).str() );
	}
};
typedef std::shared_ptr< constraints > constraints_ptr;
}
namespace de
{
class individual;
typedef std::shared_ptr< individual > individual_ptr;
class individual
{
private:
	de::DVectorPtr m_vars;
	double m_cost;
	//de::mutex m_mx;
public:
	individual( size_t varCount )
	: m_vars( std::make_shared< de::DVector >( varCount ) )
	{
	}
	individual( const de::DVector& vars )
		: m_vars( std::make_shared< de::DVector >( vars ) )
	{
	}
	void init( constraints_ptr constraints )
	{
		//assert( constraints );
		//assert( m_vars );
		//assert( m_vars->size() == constraints->size() );
		for( de::DVector::size_type j = 0; j < m_vars->size(); ++j )
			(*m_vars)[ j ] = constraints->get_rand_value( j );
	}
	double cost() const { return m_cost; }
	void ensureConstraints( constraints_ptr constraints, de::DVectorPtr origin )
	{
		//assert( constraints );
		//assert( m_vars );
		//assert( origin );
		//assert( m_vars->size() == constraints->size() );
		for( de::DVector::size_type j = 0; j < m_vars->size(); ++j )
		{
			(*m_vars)[ j ] = constraints->get_rand_value( j, (*m_vars)[ j ], (*origin)[ j ] );
		}
	}
	de::DVectorPtr vars() const { return m_vars; }
	de::myreal& operator[]( size_t index ) { return (*m_vars)[ index ]; }
	const de::myreal& operator[]( size_t index ) const { return (*m_vars)[ index ]; }
	void setCost( double cost ) { m_cost = cost; }
	bool operator<=( const individual& ind ) const
	{
		//assert( ind.size() == size() );
		return cost() <= ind.cost();
	}
	bool operator<( const individual& ind ) const
	{
		//assert( ind.size() == size() );
		return cost() < ind.cost();
	}
	bool better_or_equal( const individual_ptr ind, bool minimize ) const
	{
		//assert( ind );
		return minimize ? *this <= *ind : *ind <= *this;
	}
	bool better( const individual_ptr ind, bool minimize ) const
	{
		//assert( ind );
		return minimize ? *this < *ind : *ind < *this;
	}
	size_t size() const { return m_vars->size(); }
	std::string to_string() const
	{
		std::ostringstream os;
		os << "cost: " << cost() << ", vars: ";
		for( de::DVector::size_type j = 0; j < m_vars->size(); ++j )
		{
			os << (*m_vars)[ j ] << ", ";
		}
		return os.str();
	}
};
typedef std::queue< individual_ptr > individual_queue_base;
class individual_queue : public individual_queue_base
{
private:
	//de::mutex m_mx;
public:
	void push_back( individual_ptr ind )
	{
		//de::lock lock( m_mx );
		individual_queue_base::push( ind );
	}
	individual_ptr pop()
	{
		//de::lock lock( m_mx );
		if( !individual_queue_base::empty() )
		{
			individual_ptr p( individual_queue_base::front() );
			individual_queue_base::pop();
			return p;
		}
		else
			return individual_ptr();
	}
};
}
namespace de
{
typedef std::vector< individual_ptr > population_base;
class population : public population_base
{
public:
	population( size_t popSize, size_t varCount )
	: population_base( popSize )
	{
		//assert( popSize > 0 );
		//assert( varCount > 0 );
		init(popSize, varCount );
	}
	population( size_t popSize, size_t varCount, constraints_ptr constraints )
	: population_base( popSize )
	{
		//assert( popSize > 0 );
		//assert( varCount > 0 );
		init(popSize, varCount );
		for( population::size_type i = 0; i < size(); ++i )
			at( i )->init( constraints );
	}
	individual_ptr best( bool minimize ) const
	{
		population::size_type best( 0 );
		for( population::size_type i = 0; i < size(); ++i )
			best = at( i )->better_or_equal( at( best ), minimize )? i : best;
		return at( best );
	}
	std::string to_string()
	{
		std::string str;
		for( population::size_type i = 0; i < size(); ++i )
			str += at( i )->to_string();
		return str;
	}
private:
	void init( size_t popSize, size_t varCount )
	{
		for( population_base::size_type i = 0; i < size(); ++i )
			operator[]( i ) = std::make_shared< individual >( varCount );
	}
public:
};
	typedef std::shared_ptr< population > population_ptr;
}
namespace de
{
class processor_listener
{
public:
	virtual ~processor_listener(){}
	virtual void start( size_t index ) = 0;
	virtual void start_of( size_t index, individual_ptr individual ) = 0;
	virtual void end_of( size_t index, individual_ptr individual ) = 0;
	virtual void end( size_t index ) = 0;
	virtual void error( size_t index, const std::string& message ) = 0;
};
class null_processor_listener : public processor_listener
{
public:
	virtual void start(size_t index)
	{
	}
	virtual void start_of( size_t index, individual_ptr individual )
	{
	}
	virtual void end_of( size_t index, individual_ptr individual )
	{
	}
	virtual void end(size_t index)
	{
	}
	virtual void error(size_t index, const std::string& message )
	{
	}
};
typedef std::shared_ptr< processor_listener > processor_listener_ptr;
class objective_function_exception : public exception
{
public:
	objective_function_exception( const std::string& message )
	: exception( message.c_str() )
	{
	}
};
template< typename T >class objective_function_factory
{
public:
	typedef std::shared_ptr< T > T_ptr;
	virtual ~objective_function_factory(){}
	virtual T_ptr make() = 0;
};
template< typename T >class processor_traits
{
public:
	typedef T  value_type;
	static double run( T t, de::DVectorPtr vars ) { return t( vars); }
	static T make( T t ) { return t; }
};
template< typename T > class processor_traits< T* >
{
public:
	typedef T*  value_type;
	static double run( value_type t, de::DVectorPtr vars ) { return (*t)( vars ); }
	static value_type make( value_type t ) { return t; }
};
template< typename T > class processor_traits< std::shared_ptr< T > >
{
public:
	typedef std::shared_ptr< T > value_type;
	static double run( value_type t, de::DVectorPtr vars ) { return (*t)( vars); }
	static value_type make( value_type t ) { return t; }
};
template< typename T > class processor_traits< objective_function_factory< T >* >
{
public:
	typedef std::shared_ptr< T > value_type;
	static double run( value_type t, de::DVectorPtr vars ) { return (*t)( vars); }
	static value_type make( objective_function_factory< T >* off ) { return off->make(); }
};
template< typename T > class processor_traits< std::shared_ptr< objective_function_factory< T > > >
{
public:
	typedef std::shared_ptr< T > value_type;
	static double run( value_type t, de::DVectorPtr vars ) { return (*t)( vars); }
	static value_type make( std::shared_ptr< objective_function_factory< T > > off ) { return off->make(); }
};
template< typename T > class processor_traits< objective_function_factory< T >& >
{
public:
	typedef std::shared_ptr< T > value_type;
	static double run( value_type t, de::DVectorPtr vars ) { return (*t)( vars ); }
	static value_type make( objective_function_factory< T >& off ) { return off.make(); }
};
template < typename T > class processor //: boost::noncopyable
{
private:
	typename processor_traits< T >::value_type m_of;
	individual_queue& m_indQueue;
	processor_listener_ptr m_listener;
	size_t m_index;
	bool m_result;
public:
	processor( size_t index, T of, individual_queue& indQueue, processor_listener_ptr listener )
	: m_of( processor_traits< T >::make( of ) ), m_indQueue( indQueue ), m_result( false ), m_listener( listener ), m_index( index )
	{
		//assert( listener );
	}
	void operator()()
	{
		m_listener->start( m_index );
		m_result = false;
		try
		{
			for( individual_ptr ind = m_indQueue.pop(); ind; ind = m_indQueue.pop() )
			{
				m_listener->start_of( m_index, ind );
				double result = processor_traits< T >::run( m_of, ind->vars() );
				ind->setCost( result );
				m_listener->end_of( m_index, ind );
			}
			m_result = true;
			//BOOST_SCOPE_EXIT_TPL( (&m_index) (&m_listener) )
			{
				m_listener->end( m_index );
			}
			//BOOST_SCOPE_EXIT_END
		}
		catch( const objective_function_exception& e )
		{
			m_result = false;
			m_listener->error( m_index, e.what() );
		}
	}
	bool success() const { return m_result; }
};
class processors_exception : exception
{
public:
	processors_exception( const std::string& message )
	: exception( message.c_str() )
	{
	}
};
template< typename T > class processors
{
private:
	//typedef std::shared_ptr< boost::thread_group > thread_group_ptr;
	typedef std::shared_ptr< processor< T > > processor_ptr;
	typedef std::vector< processor_ptr > processor_vector;
	typedef std::shared_ptr< T > T_ptr;
private:
	individual_queue m_indQueue;
	processor_vector m_processors;
	//thread_group_ptr m_threads;
public:
	processors( size_t count, T of, processor_listener_ptr listener )
	{
		//assert( count > 0 );
		//assert( listener );
		for( size_t n = 0; n < count; ++n )
		{
			processor_ptr processor( std::make_shared< processor< T > >( n, of, std::ref( m_indQueue ), listener ) );
			m_processors.push_back( processors< T >::processor_ptr( processor ) ) ;
		}
	}
	void push( individual_ptr ind ) { m_indQueue.push( ind ); }
	void start()
	{
		(*m_processors[0])();
	
		//m_threads = std::make_shared< boost::thread_group >();
		//for( typename processor_vector::size_type n = 0; n < m_processors.size(); ++n )
		//{
		//	processor_ptr p( m_processors[ n  ] );
		//	boost::thread* th( new boost::thread( std::ref( *p ) ) );
		//	m_threads->add_thread( th );
		//}
	}
	void wait()
	{
		//m_threads->join_all();
		if( !m_indQueue.empty() )
			throw processors_exception( "threads ended before emptying the queue");
		if( !success() )
			throw processors_exception( "objective function error");
	}
	bool success()
	{
		for( typename processor_vector::size_type n = 0; n < m_processors.size(); ++n )
		{
			processor_ptr processor( m_processors[ n  ] );
			if( !processor->success() )
				return false;
		}
		return true;
	}
	void push( population_ptr population )
	{
		std::copy( population->begin(), population->end(), std::back_inserter( m_indQueue ) );
	}
typedef std::shared_ptr< processors< T > > processors_ptr;
};
}
namespace de
{
class mutation_strategy_arguments
{
private:
	const double m_weight;
	const double m_crossover;
	const double m_dither;
public:
	mutation_strategy_arguments( double weight, double crossover )
	: m_weight( weight ), m_crossover( crossover ), m_dither(  weight + genrand() * ( 1.0 - weight ) )
	{
	
	}
	double weight() const { return m_weight; }
	double crossover() const { return m_crossover; }
	double dither() const { return m_dither; }
};
class mutation_strategy
{
private:
	mutation_strategy_arguments m_args;
	size_t m_varCount;
protected:
	class Urn
	{
		size_t m_urn[ 5 ];
	public:
	
		Urn( size_t NP, size_t avoid )
		{
			do m_urn[ 0 ] = genintrand( 0, NP, true ) ; while( m_urn[ 0 ] == avoid ) ;
			do m_urn[ 1 ] = genintrand( 0, NP, true ) ; while( m_urn[ 1 ] == m_urn[ 0 ] || m_urn[ 1 ] == avoid );
			do m_urn[ 2 ] = genintrand( 0, NP, true ) ; while( m_urn[ 2 ] == m_urn[ 1 ] || m_urn[ 2 ] == m_urn[ 0 ] || m_urn[ 2  ] == avoid );
			do m_urn[ 3 ] = genintrand( 0, NP, true ) ; while( m_urn[ 3 ] == m_urn[ 2 ] || m_urn[ 3 ] == m_urn[ 1 ] || m_urn[ 3 ] == m_urn[ 0 ] || m_urn[ 3 ] == avoid );
		}
	
		size_t operator[]( size_t index ) const { //assert( index < 4 ); 
			return m_urn[ index ]; }
	};
public:
	virtual ~mutation_strategy()
	{
	}
	mutation_strategy( size_t varCount, const mutation_strategy_arguments& args )
	: m_args( args ), m_varCount( varCount )
	{
	}
	typedef std::tuple< individual_ptr, de::DVectorPtr > mutation_info;
	virtual mutation_info operator()( const population& pop, individual_ptr bestIt, size_t i ) = 0;
	size_t varCount() const { return m_varCount; }
	double weight() const { return m_args.weight(); }
	double crossover() const { return m_args.crossover(); }
	double dither() const { return m_args.dither(); }
};
typedef std::shared_ptr< mutation_strategy > mutation_strategy_ptr;
class mutation_strategy_1 : public mutation_strategy
{
public:
	mutation_strategy_1( size_t varCount, const mutation_strategy_arguments& args )
	: mutation_strategy( varCount, args )
	{
	}
	mutation_info operator()( const population& pop, individual_ptr bestIt, size_t i )
	{
		//assert( bestIt );
		de::DVectorPtr origin( std::make_shared< de::DVector >( varCount() ) );
		individual_ptr tmpInd( std::make_shared< individual >( *pop[ i ]->vars() ) );
		Urn urn( pop.size(), i );
	
	
		size_t j = genintrand( 0, varCount(), true );
		size_t k = 0;
		do
		{
			(*tmpInd->vars())[ j ] = (*pop[ urn[ 0 ] ]->vars() )[ j ] + weight() * ( (*pop[ urn[ 1 ] ]->vars() )[ j ] - (*pop[ urn[ 2 ] ]->vars())[ j ] );
			j = ++j % varCount();
			++k;
		} while( genrand() < crossover() && k < varCount() );
		origin = pop[ urn[ 0 ] ]->vars();
		return mutation_info( tmpInd, origin );
	}
};
class mutation_strategy_2 : public mutation_strategy
{
public:
	mutation_strategy_2( size_t varCount, const mutation_strategy_arguments& args )
	: mutation_strategy( varCount, args )
	{
	}
	mutation_info operator()( const population& pop, individual_ptr bestIt, size_t i )
	{
		//assert( bestIt );
		de::DVectorPtr origin( std::make_shared< de::DVector >( varCount() ) );
		individual_ptr tmpInd( std::make_shared< individual >( *pop[ i ]->vars() ) );
		Urn urn( pop.size(), i );
	
	
		size_t j = genintrand( 0, varCount(), true );
		size_t k = 0;
		do
		{
			(*tmpInd->vars())[ j ] = (*tmpInd->vars())[ j ] +
				weight() * ( (*bestIt->vars() )[ j ] - (*tmpInd->vars())[ j ] ) +
				weight() * ( (*pop[ urn[ 1 ] ]->vars() )[ j ] - (*pop[ urn[ 2 ] ]->vars())[ j ] );
			j = ++j % varCount();
			++k;
		} while( genrand() < crossover() && k < varCount() );
		origin = pop[ urn[ 0 ] ]->vars();
		return mutation_info( tmpInd, origin );
	}
};
class mutation_strategy_3 : public mutation_strategy
{
public:
	mutation_strategy_3( size_t varCount, const mutation_strategy_arguments& args )
	: mutation_strategy( varCount, args )
	{
	}
	mutation_info operator()( const population& pop, individual_ptr bestIt, size_t i )
	{
		//assert( bestIt );
		de::DVectorPtr origin( std::make_shared< de::DVector >( varCount() ) );
		individual_ptr tmpInd( std::make_shared< individual >( *pop[ i ]->vars() ) );
		Urn urn( pop.size(), i );
	
	
		size_t j = genintrand( 0, varCount(), true );
		size_t k = 0;
		do
		{
			double jitter = (0.0001* genrand() + weight() );
			(*tmpInd->vars())[ j ] = (*bestIt->vars() )[ j ] + jitter * ( (*pop[ urn[ 1 ] ]->vars() )[ j ] - (*pop[ urn[ 2 ] ]->vars())[ j ] );
			j = ++j % varCount();
			++k;
		} while( genrand() < crossover() && k < varCount() );
		origin = pop[ urn[ 0 ] ]->vars();
		return mutation_info( tmpInd, origin );
	}
};
class mutation_strategy_4 : public mutation_strategy
{
public:
	mutation_strategy_4( size_t varCount, const mutation_strategy_arguments& args )
	: mutation_strategy( varCount, args )
	{
	}
	mutation_info operator()( const population& pop, individual_ptr bestIt, size_t i )
	{
		//assert( bestIt );
		de::DVectorPtr origin( std::make_shared< de::DVector >( varCount() ) );
		individual_ptr tmpInd( std::make_shared< individual >( *pop[ i ]->vars() ) );
		Urn urn( pop.size(), i );
	
	
		size_t j = genintrand( 0, varCount(), true );
		size_t k = 0;
		double factor( weight() + genrand() * ( 1.0 - weight() ) );
		do
		{
			double jitter = (0.0001* genrand() + weight() );
			(*tmpInd->vars())[ j ] = (*pop[ urn[ 0 ] ]->vars() )[ j ] +
				factor * ( (*pop[ urn[ 1 ] ]->vars() )[ j ] - (*pop[ urn[ 2 ] ]->vars())[ j ] );
			j = ++j % varCount();
			++k;
		} while( genrand() < crossover() && k < varCount() );
		origin = pop[ urn[ 0 ] ]->vars();
		return mutation_info( tmpInd, origin );
	}
};
class mutation_strategy_5 : public mutation_strategy
{
public:
	mutation_strategy_5( size_t varCount, const mutation_strategy_arguments& args )
	: mutation_strategy( varCount, args )
	{
	}
	mutation_info operator()( const population& pop, individual_ptr bestIt, size_t i )
	{
		//assert( bestIt );
		de::DVectorPtr origin( std::make_shared< de::DVector >( varCount() ) );
		individual_ptr tmpInd( std::make_shared< individual >( *pop[ i ]->vars() ) );
		Urn urn( pop.size(), i );
	
		size_t j = genintrand( 0, varCount(), true );
		size_t k = 0;
		do
		{
			(*tmpInd->vars())[ j ] = (*pop[ urn[ 0 ] ]->vars() )[ j ] + dither() * ( (*pop[ urn[ 1 ] ]->vars() )[ j ] - (*pop[ urn[ 2 ] ]->vars() )[ j ] );
			j = ++j % varCount();
			++k;
		} while( genrand() < crossover() && k < varCount() );
		origin = pop[ urn[ 0 ] ]->vars();
		return mutation_info( tmpInd, origin );
	}
};
}
namespace de
{
class selection_strategy
{
public:
	virtual ~selection_strategy(){}
	virtual void operator()( population_ptr& pop1, population_ptr& pop2, individual_ptr& bestInd, bool minimize ) = 0;
};
typedef std::shared_ptr< selection_strategy > selection_strategy_ptr;
class best_parent_child_selection_strategy : public selection_strategy
{
public:
	void operator()( population_ptr& pop1, population_ptr& pop2, individual_ptr& bestInd, bool minimize )
	{
		//assert( pop1 );
		//assert( pop2 );
		//assert( pop1->size() == pop2->size() );
		sort_across( *pop1, *pop2, minimize );
	
		bestInd = (*pop1)[ 0 ];
	}
private:
	class individual_compare
	{
	private:
		const bool m_minimize;
	public:
		individual_compare( bool minimize )
		: m_minimize( minimize )
		{
		}
		bool operator()( individual_ptr ind1, individual_ptr ind2 )
		{
			//assert( ind1 && ind2 );
			return ind1->better( ind2, m_minimize );
		}
	};
	void sort_across( population& v1, population& v2, bool minimize  )
	{
		v1.insert( v1.end(), v2.begin(), v2.end() );
		v2.clear();
		std::sort( v1.begin(), v1.end(), individual_compare( minimize ) );
		v2.insert( v2.end(), v1.begin() + v1.size() / 2, v1.end() );
		v1.erase( v1.begin() + v1.size()/2, v1.end() );
	}
};
class tournament_selection_strategy : public selection_strategy
{
public:
	void operator()( population_ptr& pop1, population_ptr& pop2, individual_ptr& bestInd, bool minimize )
	{
		//assert( pop1 );
		//assert( pop2 );
		//assert( pop1->size() == pop2->size() );
		for( size_t i = 0; i < pop1->size(); ++i )
		{
			individual_ptr crt( (*pop2)[ i ] );
			if( crt->better_or_equal( (*pop1 )[ i ], minimize ) )
			{
				if( crt->better_or_equal( bestInd, minimize ) )
					bestInd = crt;
			}
			else
				(*pop2)[ i ] = (*pop1)[ i ];
		}
		std::swap( pop1, pop2 );
	}
};
}
namespace de
{
class termination_strategy
{
public:
	virtual ~termination_strategy(){}
	virtual bool event( individual_ptr best, size_t genCount ) = 0;
};
typedef std::shared_ptr< termination_strategy > termination_strategy_ptr;
class max_gen_termination_strategy : public termination_strategy
{
private:
	const size_t m_maxGen;
public:
	max_gen_termination_strategy( size_t maxGen )
		: m_maxGen( maxGen )
	{
	}
	virtual bool event( individual_ptr best, size_t genCount )
	{
		return genCount < m_maxGen;
	}
};
}
namespace de
{
class listener
{
public:
	virtual ~listener() {}
	virtual void start() = 0;
	virtual void end() = 0;
	virtual void error() = 0;
	virtual void startGeneration( size_t genCount ) = 0;
	virtual void endGeneration( size_t genCount, individual_ptr bestIndGen, individual_ptr bestInd ) = 0;
	virtual void startSelection( size_t genCount ) = 0;
	virtual void endSelection( size_t genCount ) = 0;
	virtual void startProcessors( size_t genCount ) = 0;
	virtual void endProcessors( size_t genCount ) = 0;
};
typedef std::shared_ptr< listener > listener_ptr;
class null_listener : public listener
{
public:
	virtual void start(){}
	virtual void end(){}
	virtual void error(){}
	virtual void startGeneration( size_t genCount ){}
	virtual void endGeneration( size_t genCount, individual_ptr bestIndGen, individual_ptr bestInd ){}
	virtual void startSelection( size_t genCount ){}
	virtual void endSelection( size_t genCount ){}
	virtual void startProcessors( size_t genCount ){}
	virtual void endProcessors( size_t genCount ){}
};
}
namespace de
{
class differential_evolution_exception
{
};
template< typename T > class differential_evolution
{
private:
	const size_t m_varCount;
	const size_t m_popSize;
	population_ptr m_pop1;
	population_ptr m_pop2;
	individual_ptr m_bestInd;
	constraints_ptr m_constraints;
	typename processors< T >::processors_ptr m_processors;
	termination_strategy_ptr m_terminationStrategy;
	selection_strategy_ptr m_selectionStrategy;
	mutation_strategy_ptr m_mutationStrategy;
	listener_ptr m_listener;
	const bool m_minimize;
public:
	differential_evolution( size_t varCount, size_t popSize, typename processors< T >::processors_ptr processors, constraints_ptr constraints, bool minimize,
						   termination_strategy_ptr terminationStrategy, selection_strategy_ptr selectionStrategy,
							mutation_strategy_ptr mutationStrategy, de::listener_ptr listener )
	try
		: m_varCount( varCount ), m_popSize( popSize ), m_pop1( std::make_shared< population >( popSize, varCount, constraints ) ),
		m_pop2( std::make_shared< population >( popSize, varCount ) ), m_bestInd( m_pop1->best( minimize ) ),
		m_constraints( constraints ), m_processors( processors ), m_minimize( minimize ), m_terminationStrategy( terminationStrategy ),
		m_listener( listener ), m_selectionStrategy( selectionStrategy ), m_mutationStrategy( mutationStrategy )
	{
		//assert( processors );
		//assert( constraints );
		//assert( terminationStrategy );
		//assert( selectionStrategy );
		//assert( listener );
		//assert( mutationStrategy );
		//assert( popSize > 0 );
		//assert( varCount > 0 );
	
	
		processors->push( m_pop1 );
		processors->start();
		processors->wait();
	}
	catch( const processors_exception&)
	{
		throw differential_evolution_exception();
	}
	virtual ~differential_evolution(void)
	{
	}
	void run()
	{
		try
		{
			m_listener->start();
			individual_ptr bestIndIteration( m_bestInd );
			for( size_t genCount = 0 ; m_terminationStrategy->event( m_bestInd, genCount ); ++genCount )
			{
				m_listener->startGeneration( genCount );
				for( size_t i = 0; i < m_popSize; ++i)
				{
					mutation_strategy::mutation_info mutationInfo( ( *m_mutationStrategy)( *m_pop1, bestIndIteration, i ) );
					individual_ptr tmpInd( std::get< 0 >( mutationInfo ) );
					tmpInd->ensureConstraints( m_constraints, std::get< 1 >( mutationInfo ) );
				
					m_processors->push( tmpInd );
				
				
					(*m_pop2)[ i ] = tmpInd;
				}
				m_listener->startProcessors( genCount );
				m_processors->start();
				m_processors->wait();
				m_listener->endProcessors( genCount );
			
				m_listener->startSelection( genCount );
				(*m_selectionStrategy)( m_pop1, m_pop2, m_bestInd, m_minimize );
				bestIndIteration = m_bestInd;
				m_listener->endSelection( genCount );
				m_listener->endGeneration( genCount, bestIndIteration, m_bestInd );
			}
			//BOOST_SCOPE_EXIT_TPL( (m_listener) )
			{
				m_listener->end();
			}
			//BOOST_SCOPE_EXIT_END
		}
		catch( const processors_exception& )
		{
			m_listener->error();
			throw differential_evolution_exception();
		}
	}
	individual_ptr best() const { return m_bestInd; }
};
}

// \/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/
// \/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/
// \/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/
//-----------------------------------------------------------------------------



























//-----------------------------------------------------------------------------
// /\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\
// /\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\
// /\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\
// МОЙ КОД МОЙ КОД МОЙ КОД МОЙ КОД МОЙ КОД МОЙ КОД МОЙ КОД МОЙ КОД МОЙ КОД МОЙ 

//-----------------------------------------------------------------------------
realreal calcTriangleArea(const point& a, const point& b, const point& c) {
	// Считает площадь треугольника, заданного тремя точками
	return fabs((a.x-c.x)*(b.y-c.y)-(a.y-c.y)*(b.x-c.x))/2.0;
}

//-----------------------------------------------------------------------------
realreal calcLength(const point& d) {
	// Считает длину точки d
	return sqrt(d.x*d.x + d.y*d.y);
}

//-----------------------------------------------------------------------------
point calcMiddle(const point& a, const point& b) {
	// Считает точку посередине между a и b
	return {(a.x+b.x)/2.0, (a.y+b.y)/2.0};
}

//-----------------------------------------------------------------------------
realreal calcDistance(const point& a, const point& b) {
	// Считает расстояние между двумя точками
	return calcLength({a.x-b.x, a.y-b.y});
}

//-----------------------------------------------------------------------------
bool intersectLines(const line& l1, const line& l2, realreal& t1, realreal& t2, point& result) {
	// Пересекает отрезки
	realreal a = l2.b.x-l2.a.x;
	realreal b = -(l1.b.x-l1.a.x);
	realreal a1 = l2.b.y-l2.a.y;
	realreal b1 = -(l1.b.y-l1.a.y);

	realreal e = a * b1 - b * a1;
	if (fabs(e) < line_around)
		return false;
	realreal c = l1.a.x - l2.a.x;
	realreal c1 = l1.a.y - l2.a.y;
	t1 = -c * a1 + a * c1;
	t2 = b1 * c - b * c1;
	t1 /= e;
	t2 /= e;

	result.x = l1.a.x - b * t1;
	result.y = l1.a.y - b1 * t1;
	return true;
}

//-----------------------------------------------------------------------------
realreal getAngle(const point& center, const point& p) {
	// Возвращает на какой угол повернута точка p относительно center
	return atan2(center.y-p.y, center.x-p.x) + mypi;
}

//-----------------------------------------------------------------------------
template<class T, class F>
void travelPolygonEdges(vector<T> mas, F f) {
	// Обходит все ребра многоугольника
	for (int i = 0; i < mas.size()-1; ++i)
		f(i, i+1);
	f(mas.size()-1, 0);
}

#ifdef USE_TWG
//-----------------------------------------------------------------------------
void drawPoint(twg::ImageDrawing& img, twg::Point_d a, double r, twg::Color clr) {
	using namespace twg;
	auto poly = computeEllipse(Point_d(r, r));
	poly.move(a);
	img.setBrush(clr);
	img.drawPolygon(poly);
}
#endif

//-----------------------------------------------------------------------------
realreal calcRadialArea(const point& p) {
	// Если точка не внутри полигона, то она нас не интересует, и, следовательно, покрываемая ей площадь равна нулю
	if (!pointInPolygon(p))
		return 0;

	#ifdef USE_TWG
	using namespace twg;
	double imgsize = 2000;
	double border = 20;
	ImageDrawing_aa img(Point_d(imgsize, imgsize));
	img.clear(White);

	Point_d mx(double((imgsize-border)/(in.max.x - in.min.x)), double((imgsize-border)/(in.max.y - in.min.y)));
	Point_d off(double(-in.min.x), double(-in.min.y));
	off = off * mx;
	off += Point_d(border/2.0, border/2.0);
	Polygon_d poly1;
	for (auto i : in.p) {
		poly1.array.push_back(Point_d(i)*mx + off);
		drawPoint(img, poly1.array.back(), 5, Black);
	}
	img.setPen(Pen(2, Black));
	img.setBrush(Brush(setAlpha(Black, 20)));
	img.drawPolygon(poly1);
	img.drawPolyline(poly1);
	#endif

	// Сортируем все вершины по углу
	vector<pair<realreal, int>> mas(in.p.size());
	for (int i = 0; i < in.p.size(); ++i)
		mas[i] = pair<realreal, int>(getAngle(p, in.p[i]), i);
	sort(mas.begin(), mas.end());

	// Удаляем слишком близкие друг к другу вершины
	start_delete:
	for (int i = 0; i < mas.size() - 1; ++i) {
		if (fabs(mas[i].first - mas[i+1].first) < delete_around) {
			mas.erase(mas.begin() + i + 1);
			goto start_delete;
		}
	}
	if (fabs(mas[mas.size()-1].first - (2 * mypi + mas[0].first)) < delete_around)
		mas.erase(mas.begin() + mas.size() - 1);

	realreal sumArea = 0;
	travelPolygonEdges(mas, [&] (int c1, int c2) {
		static vector<line> segments(50);
		segments.clear();

		// Перебираем все ребра
		travelPolygonEdges(in.p, [&] (int p1, int p2) {
			point s1, s2;
			realreal t11, t12, t21, t22;
			// Находим точки, где линии сектора пересекают ребро
			if (intersectLines({in.p[p1], in.p[p2]}, {p, in.p[mas[c1].second]}, t11, t12, s1)
			&& intersectLines({in.p[p1], in.p[p2]}, {p, in.p[mas[c2].second]}, t21, t22, s2) 
			&& (fabs(t11 - 0) < zero_around || fabs(t11 - 1) < zero_around || (t11 > 0 && t11 < 1)) && (fabs(t12 - 0) < zero_around || t12 > 0) 
			&& (fabs(t21 - 0) < zero_around || fabs(t21 - 1) < zero_around || (t21 > 0 && t21 < 1)) && (fabs(t22 - 0) < zero_around || t22 > 0)) {
				segments.push_back({s1, s2});

				#ifdef USE_TWG
				//img.setPen(Pen(1, Green));
				//img.drawLine(Point_d(s1)*mx + off, Point_d(s2)*mx + off);
				#endif
			}
		});

		// Перебираем все отрезки и находим ближайший
		int min = 0;
		if (segments.size() != 0) {
			realreal minDist = calcDistance(p, calcMiddle(segments[0].a, segments[0].b));
			for (int i = 1; i < segments.size(); ++i) {
				realreal dist = calcDistance(p, calcMiddle(segments[i].a, segments[i].b));
				if (dist < minDist) {
					min = i;
					minDist = dist;
				}
			}

			#ifdef USE_TWG
			Polygon_d poly;
			poly.array.push_back(p);
			poly.array.push_back(segments[min].a);
			poly.array.push_back(segments[min].b);
			poly.scale(mx);
			poly.move(off);
			img.setBrush(Brush(setAlpha(Orange, 64)));
			img.setPen(Pen(1, Orange));
			img.drawPolygon(poly);
			img.drawPolyline(poly);
			#endif

			// Считаем площадь центра с этим отрезком
			sumArea += calcTriangleArea(p, segments[min].a, segments[min].b);

			#ifdef USE_TWG
			//saveToBmp(&img, L"b.bmp");
			#endif
		} else {
			//cout << "something went wrong" << endl;
		}
	});

	#ifdef USE_TWG
	saveToBmp(&img, L"a.bmp");
	#endif

	return sumArea;
}

//-----------------------------------------------------------------------------
// pair<point, point> calcDerivatives(const point& p) {
// 	// Считает градиент и второй градиент функции 
// 	// Использовались самые лучшие найденные формулы

// 	point firstDerivative, secondDerivative;
// 	realreal step = 1e-4;
// 	realreal x0 = calcRadialArea(p);
// 	realreal x1 = calcRadialArea({p.x+step, p.y});
// 	realreal x2 = calcRadialArea({p.x-step, p.y});
// 	realreal x3 = calcRadialArea({p.x+2*step, p.y});
// 	realreal x4 = calcRadialArea({p.x-2*step, p.y});
// 	realreal y0 = x0;
// 	realreal y1 = calcRadialArea({p.x, p.y+step});
// 	realreal y2 = calcRadialArea({p.x, p.y-step});
// 	realreal y3 = calcRadialArea({p.x, p.y+2*step});
// 	realreal y4 = calcRadialArea({p.x, p.y-2*step});

// 	//firstDerivative.x = (x1-x0)/(step);
// 	//firstDerivative.y = (y1-x0)/(step);

// 	firstDerivative.x = (-2*x2-3*x0+6*x1-x3)/(6.0*step);
// 	firstDerivative.y = (-2*y2-3*y0+6*y1-y3)/(6.0*step);

// 	//secondDerivative.x = (x1-2*x0+x2)/(step*step);
// 	//secondDerivative.y = (y1-2*y0+y2)/(step*step);

// 	secondDerivative.x = (-x3+16*x1-30*x0+16*x2-x4)/(12*step*step);
// 	secondDerivative.y = (-y3+16*y1-30*y0+16*y2-y4)/(12*step*step);

// 	return {firstDerivative, secondDerivative};
// }

//-----------------------------------------------------------------------------
// point calcGradientDescent(const point& start, realreal eps, int maxiter) {
// 	// Типо считает градиентный спуск, но на самом деле не работает
// 	point x = start;
// 	realreal step = 1e-3;
// 	realreal residual = 0;
// 	for (int i = 0; i < maxiter; i++) {
// 		auto d = calcDerivatives(x);
// 		x.x = x.x - step * d.second.x;
// 		x.y = x.y - step * d.second.y;
// 		bool is = pointInPolygon(x);
// 		residual = d.second.x * d.second.x + d.second.y * d.second.y;
// 		if (residual < eps)
// 			return x;
// 	}
// 	return x;
// }

//-----------------------------------------------------------------------------
void readData(void) {
	// Вводит все необходимые в программе данные
	cin >> in.n;
	in.p.resize(in.n);
	for (int i = 0; i < in.n; ++i)
		cin >> in.p[i].x >> in.p[i].y;

	// Находим минимум и максимум среди точек, чтобы у эволюции были границы
	in.min.x = 1010;
	in.min.y = 1010;
	in.max.x = -1010;
	in.max.y = -1010;
	for (const auto& i : in.p) {
		if (i.x < in.min.x)
			in.min.x = i.x;
		if (i.y < in.min.y)
			in.min.y = i.y;
		if (i.x > in.max.x)
			in.max.x = i.x;
		if (i.y > in.max.y)
			in.max.y = i.y;
	}
}

double evolveToResult(void);

//-----------------------------------------------------------------------------
void test(string str, realreal result) {
	static int count = 1;
	// Функция для передачи строки на потоковый ввод, чтобы легко тестировать программу
	string* str1 = new string(str);
	istringstream* iss = new istringstream(*str1);
	cin.rdbuf(iss->rdbuf());

	//-------------------------------------------------------------------------
	// Вводим данные
	readData();

	#ifdef USE_TWG
	ofstream fout("a.txt");
	realreal x1("4.8893442876655335");
	realreal y1("-7.0122957669693466"); 
	fout << setprecision(16) << x1 << endl << y1 << endl << calcRadialArea({x1, y1}) << endl;
	fout.close();
	system("pause");
	#endif

	// Запускаем эволюцию до результата и выводим её результат
	realreal evolved = evolveToResult();
	cout << fixed << count << ": " << setprecision(3) << evolved << " " << result << " | " << evolved - result << endl;

	count++;
}

void test1(string str) {
	// Функция для передачи строки на потоковый ввод, чтобы легко тестировать программу
	string* str1 = new string(str);
	istringstream* iss = new istringstream(*str1);
	cin.rdbuf(iss->rdbuf());
}
// \/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/
// \/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/
// \/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/
//-----------------------------------------------------------------------------

//-----------------------------------------------------------------------------
// /\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\
// /\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\
// /\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\
// Мой код, который использует дифференциальную эволюцию

using namespace de;

struct my_function {
	double operator()( de::DVectorPtr args ) {
		return double(calcRadialArea({realreal(double((*args)[0])), realreal(double((*args)[1]))}));
	}
};

// Класс вывода данных о поколенях на экран, используется для отладки
class my_listener : public listener
{
public:
	virtual void start(){}
	virtual void end(){}
	virtual void error(){}
	virtual void startGeneration( size_t genCount ){}
	virtual void endGeneration( size_t genCount, individual_ptr bestIndGen, individual_ptr bestInd ){
		cout << fixed << setprecision(3);
		std::cout << "Gen: " << genCount << ", " << (*bestInd).cost() << std::endl;
		cout << setprecision(16);
		cout << endl << (*bestInd).vars().get()->operator[](0) << " " << (*bestInd).vars().get()->operator[](1) << endl;
	}
	virtual void startSelection( size_t genCount ){}
	virtual void endSelection( size_t genCount ){}
	virtual void startProcessors( size_t genCount ){}
	virtual void endProcessors( size_t genCount ){}
};

double evolveToResult(void) {
	try {
		const int vars_count = 2;
		const int population_size = 100;
		const int generations_max = 500;
		const int threads_count = 1;
		const double first_parameter = 1.6;
		const double second_parameter = 0.3;
		constraints_ptr constraints( std::make_shared< constraints >( vars_count , -1.0e6, 1.0e6 ) );
		(*constraints)[ 0 ] = std::make_shared< real_constraint >( double(in.min.x), double(in.max.x) );
		(*constraints)[ 1 ] = std::make_shared< real_constraint >( double(in.min.y), double(in.max.y) );
		my_function of;
		listener_ptr listener( std::make_shared< null_listener >() );
		processor_listener_ptr processor_listener( std::make_shared< null_processor_listener >() );
		processors< my_function >::processors_ptr _processors( std::make_shared< processors< my_function > >( threads_count, std::ref( of ), processor_listener ) );
		termination_strategy_ptr terminationStrategy( std::make_shared< max_gen_termination_strategy >( generations_max ) );
		selection_strategy_ptr selectionStrategy( std::make_shared< best_parent_child_selection_strategy >() );
		mutation_strategy_arguments mutation_arguments( first_parameter, second_parameter );
		mutation_strategy_ptr mutationStrategy( std::make_shared< mutation_strategy_1 >( vars_count, mutation_arguments ) );
		differential_evolution< my_function > de( vars_count, population_size, _processors, constraints, false, terminationStrategy, selectionStrategy, mutationStrategy, listener );

		de.run();

		cout << setprecision(16);
		cout << (*de.best()).vars().get()->operator[](0) << " " << (*de.best()).vars().get()->operator[](1) << endl;
		return (*de.best()).cost();
	}
	catch( const de::exception& e ) {
		std::cout << "an error occurred: " << e.what();
		return -1;
	}
}
// \/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/
// \/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/
// \/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/
//-----------------------------------------------------------------------------

//-----------------------------------------------------------------------------
//-----------------------------------------------------------------------------
//-----------------------------------------------------------------------------

#ifdef USE_TWG
int CALLBACK WinMain(HINSTANCE hInstance, HINSTANCE hPrevInstance, LPSTR lpCmdLine, int nCmdShow) {
#else
int main() {
#endif

	//-------------------------------------------------------------------------
	// Куча тестов
	// cout << fixed << setprecision(3);
	// test("4\n0 0 1 0 1 1 0 1", 1.000); // 1
	// test("3\n0 0 1 1 0 1", 0.500); // 2
	// test("4\n0 0 5 5 10 0 5 10", 25.000); // 3
	// test("8\n0 0 200 0 100 50 200 75 100 100 200 150 100 200 0 200", 30000.000); // 4
	// test("12\n0 0 -100 0 -100 -10 50 -10 50 0 200 0 100 50 200 75 100 100 200 150 100 200 0 200", 30533.333); // 5
	// test("50\n-17 -25 -3 -38 23 -40 -19 -43 29 -54 48 -45 45 -55 65 -53 46 -25 67 -30 74 -41 62 -19 41 -16 29 -27 42 -31 49 -35 40 -39 29 -35 19 -27 18 -14 32 -4 50 -2 55 2 49 18 33 25 12 33 20 38 31 40 34 47 22 51 8 53 -8 53 -11 48 3 44 -3 35 -25 30 -37 12 -22 13 4 17 3 7 -12 3 -45 3 -57 -4 -46 -10 -33 -8 -19 -8 -47 -16 -54 -22 -30 -26 -28 -23", 3960.291); // 6
	// test("50\n-17 -25 3 -9 23 -40 -19 -43 29 -54 48 -45 45 -55 65 -53 46 -25 67 -30 74 -41 62 -19 41 -16 29 -27 42 -31 49 -35 40 -39 29 -35 19 -27 7 -5 32 -4 50 -2 20 6 49 18 20 10 12 33 20 38 31 40 34 47 16 40 8 53 12 42 -11 48 12 23 -32 20 9 16 -37 12 -24 10 17 13 3 7 -12 3 16 -1 -57 -4 -46 -10 -33 -8 3 -7 -47 -16 -54 -22 -30 -26 -17 -13", 938.659); // 7
	// test("16\n-1000 -533 -226 -723 252 -414 487 -230 524 -31 788 -825 969 -401 1000 -300 1000 825 -310 970 -945 331 -990 803 -993 442 -998 994 -999 30 -1000 952", 2680128.583); // 8
	// test("6\n0 0 5 1 10 0 10 10 0 10 1 5", 90.000); // 9
	// test("8\n-10 0 0 -50 10 0 50 10 10 20 0 60 -10 20 -50 10", 2100.000); // 10
	// test("8\n-80 -50 -60 -50 -60 -40 60 -40 60 -50 80 -50 80 -30 -80 -30", 1816.667); // 11
	// test("50\n-300 -900 3 -9 23 -40 0 -1000 29 -54 48 -45 45 -55 500 -800 46 -25 70 -19 52 -26 1000 -10 41 -16 32 -2 42 -31 49 -35 40 -39 29 -35 19 -27 7 -5 32 0 1000 60 20 6 820 440 20 10 12 33 20 38 31 40 200 1000 16 40 8 53 12 42 -900 900 12 23 -32 20 9 16 -1000 40 -24 10 17 13 3 7 -12 3 16 -1 -1000 0 -46 -10 -33 -8 3 -7 -47 -16 -54 -22 -30 -26 -17 -13", 15912.544); // 12
	// test("41\n-24 10 -13 13 3 7 -12 3 16 -1 -1000 0 -46 -10 -33 -8 3 -7 -47 -16 -54 -22 -30 -26 -17 -13 -300 -900 23 -40 0 -1000 29 -54 45 -55 500 -800 52 -26 1000 -10 57 -17 42 -24 36 -12 54 -4 1000 60 44 12 820 440 39 25 41 33 20 38 31 40 200 1000 16 40 8 53 12 42 -900 900 12 23 -32 20 -21 15 -1000 40", 53322.557); // 13
	// test("6\n-1000 -1000 -999 -1000 1000 1000 999 1000 -990 -990 -1000 -990", 2045.000); // 14
	// test("26\n-1000 -874 -591 -150 445 -33 600 -18 617 -168 822 -6 988 -271 993 -434 994 -990 995 -591 996 -864 997 -333 1000 -273 1000 970 33 850 -882 339 -972 582 -984 357 -991 629 -993 785 -995 574 -996 610 -997 765 -998 191 -999 9 -1000 194", 1797910.836); // 15
	// test("40\n-1000 -562 -585 -612 -520 -927 430 -890 603 -525 713 -954 909 -950 910 -289 943 -580 947 -728 989 -587 990 -545 991 -383 992 -935 993 -569 994 -51 995 -334 996 -451 997 -507 1000 -220 1000 375 -743 753 -797 795 -875 647 -885 461 -935 333 -963 558 -978 194 -983 392 -987 754 -991 423 -992 567 -993 879 -994 336 -995 51 -996 454 -997 921 -998 700 -999 226 -1000 633", 2717774.882); // 16
	// test("32\n-1000 -346 142 -185 172 -835 842 -940 956 -92 976 -558 985 -254 987 -946 988 -435 989 -988 992 -733 994 -482 995 -588 996 -344 997 -110 1000 -571 1000 474 786 676 -820 890 -875 477 -987 10 -989 665 -991 75 -992 676 -993 14 -994 525 -995 710 -996 193 -997 364 -998 57 -999 623 -1000 404", 2436001.444); // 17
	// test("10\n-1000 -411 -290 -445 656 -314 824 -632 1000 -253 1000 889 -694 97 -882 120 -906 435 -1000 298", 1691895.570); // 18
	// test("32\n-1000 -683 -203 -184 -91 -330 -44 -87 325 -210 723 -57 948 -167 961 -116 970 -946 986 -770 993 -864 994 -433 995 -208 996 -768 997 -500 1000 -247 1000 181 -651 342 -847 109 -883 797 -977 11 -979 703 -990 970 -992 517 -993 745 -994 359 -995 421 -996 784 -997 504 -998 833 -999 301 -1000 703", 1029065.711); // 19
	// test("18\n-1000 -908 102 -932 819 -941 896 -250 902 -207 979 -699 993 -874 995 -681 1000 -767 1000 102 815 954 -125 484 -599 123 -885 261 -934 875 -959 80 -969 672 -1000 14", 2779446.433); // 20
	// test("46\n-1000 -436 -231 -441 974 -204 975 -564 978 -110 979 -631 982 -592 983 -271 984 -950 985 -819 986 -395 987 -188 988 -466 989 -145 990 -803 991 -151 992 -987 993 -152 994 -832 995 -510 996 -855 997 -400 1000 -452 1000 555 -242 199 -346 618 -475 972 -976 516 -983 16 -984 575 -985 461 -986 328 -987 342 -988 316 -989 756 -990 527 -991 888 -992 819 -993 202 -994 270 -995 611 -996 476 -997 851 -998 494 -999 469 -1000 932", 1725203.784); // 21
	// test("40\n-1000 -675 688 -322 735 -718 897 -355 954 -154 974 -911 976 -585 978 -337 984 -703 988 -578 989 -187 990 -904 991 -841 992 -974 993 -861 994 -668 995 -346 996 -505 997 -442 1000 -526 1000 833 -609 987 -663 468 -932 704 -948 710 -954 701 -966 902 -985 691 -987 250 -989 755 -991 998 -992 628 -993 518 -994 314 -995 482 -996 686 -997 80 -998 140 -999 987 -1000 956", 2664367.219); // 22
	// test("42\n-1000 -329 -597 -912 -426 -8 -39 -605 3 -208 844 -736 938 -969 972 -893 985 -22 987 -437 988 -872 989 -403 990 -68 991 -176 992 -220 993 -336 994 -725 995 -820 996 -624 997 -733 1000 -654 1000 501 89 712 -795 714 -843 641 -961 502 -973 480 -975 372 -977 177 -979 605 -984 176 -987 449 -989 260 -992 193 -993 391 -994 531 -995 526 -996 609 -997 389 -998 996 -999 781 -1000 180", 2266304.288); // 23
	// test("48\n-1000 -86 -207 -987 135 -180 479 -891 882 -711 950 -209 955 -36 966 -35 974 -536 980 -665 984 -673 986 -436 987 -568 988 -501 989 -266 990 -837 991 -282 992 -940 993 -647 994 -92 995 -357 996 -734 997 -764 1000 -464 1000 141 306 313 66 304 -553 293 -813 490 -948 835 -963 856 -966 906 -971 71 -979 524 -986 426 -988 321 -989 93 -990 685 -991 118 -992 441 -993 188 -994 752 -995 19 -996 433 -997 244 -998 220 -999 727 -1000 765", 1765326.731); // 24
	// test("40\n-1000 -757 155 -734 923 -877 971 -658 976 -273 982 -163 984 -774 986 -247 987 -465 988 -862 989 -886 990 -885 991 -384 992 -87 993 -359 994 -727 995 -283 996 -351 997 -166 1000 -982 1000 216 -812 469 -981 582 -984 314 -985 54 -986 411 -987 231 -988 537 -989 923 -990 468 -991 312 -992 886 -993 728 -994 387 -995 710 -996 129 -997 647 -998 850 -999 944 -1000 99", 2239935.962); // 25
	
	test1("50\n-17 -25 -3 -38 23 -40 -19 -43 29 -54 48 -45 45 -55 65 -53 46 -25 67 -30 74 -41 62 -19 41 -16 29 -27 42 -31 49 -35 40 -39 29 -35 19 -27 18 -14 32 -4 50 -2 55 2 49 18 33 25 12 33 20 38 31 40 34 47 22 51 8 53 -8 53 -11 48 3 44 -3 35 -25 30 -37 12 -22 13 4 17 3 7 -12 3 -45 3 -57 -4 -46 -10 -33 -8 -19 -8 -47 -16 -54 -22 -30 -26 -28 -23");

	//-------------------------------------------------------------------------
	// Вводим данные
	readData();

	cout << fixed << setprecision(3) << calcRadialArea({4.889344287665533, -7.012295766969347}) << endl;

	// Запускаем эволюцию до результата и выводим её результат
	//cout << fixed << setprecision(3) << evolveToResult();

	//-------------------------------------------------------------------------
	// Код для проверки всех пересечений всех сторон многоугольника. Была теория о том, что максимум находится на одной из таких точек. Но это оказалось не так.
	vector<pair<double, point>> mas;
	cout << fixed << setprecision(5);
	travelPolygonEdges(in.p, [&] (int a1, int a2) {
		travelPolygonEdges(in.p, [&] (int b1, int b2) {
			if (a1 != b1 && a1 != b2 && a2 != b1 && a2 != b2) {
				double t1, t2;
				point s;
				if (intersectLines(
				{in.p[a1], in.p[a2]}, 
				{in.p[b1], in.p[b2]}, 
					t1, t2, s
				))
				if (pointInPolygon(s))
					mas.push_back(pair<double, point>(calcRadialArea(s), s));
			}
		});
	});
	sort(mas.begin(), mas.end(), [] (auto& a, auto& b) -> bool {
		return a.first > b.first;
	});
	cout << fixed << setprecision(3) << mas[0].first;
	/*for (auto i : mas) {
		cout << setw(20) << i.second.x << " " << setw(20) << i.second.y << " " << setw(20) << i.first << endl;
		system("pause");
	}*/

	//-------------------------------------------------------------------------
	// Код для генерации данных, которые потом можно визуализировать.
	// В частности эти данные можно визуализировать так: gnuplot -> set pm3d map -> splot "a.dat" with pm3d
	// double xmax = 0, ymax = 0, hmax = 0;
	// ofstream fout("a.dat");
	// fout << fixed;
	// double sizex = in.max.x-in.min.x, sizey = in.max.y-in.min.y, aroundx = in.min.x + sizex/2.0, aroundy = in.min.y + sizey/2.0;
	// double count = 100;
	// double border = 0.05 * count;
	// for (double i = -border; i <= count+border; i++) {
	// 	for (double j = -border; j <= count+border; j++) {
	// 		double x = aroundx - sizex/2.0 + i/count * sizex;
	// 		double y = aroundy - sizey/2.0 + j/count * sizey;
	// 		if (pointInPolygon({x, y})) {
	// 			double h = calcRadialArea({x, y});
	// 			fout << i/count << " " << j/count << " " << h/1e6 << endl;
	// 			if (h > hmax) {
	// 				xmax = x;
	// 				ymax = y;
	// 				hmax = h;
	// 			}
	// 		} else {
	// 			fout << i/count << " " << j/count << " " << 0 << endl;
	// 		}
	// 	}
	// 	fout << endl;
	// }
	// fout.close();
	// cout << "Max from data: ";
	// cout << setprecision(8) << fixed << xmax << " " << ymax << " " << hmax << endl;

	// cout << endl; system("pause");
}

/*
Здравствуйте, решал [h]1 задачу[/h] на досуге, и обнаружил, что в ней [h]6 тест[/h] может быть неправильным.

Я нашел точку: 
[h]x = 4.889344287665533
y = -7.012295766969347[/h]

Где покрываемая площадь равна [h]3968.162118618966[/h], хотя в ответах к тесту говорится, что максимальная [h]3960.291[/h].

Я думал, может я что-то неправильно сделал, и проверил самые узкие места: 
1. Алгоритм нахождения треугольников, которые покрывает охранная система.
2. Точность вычислений.
3. Точка находится вне полигона.

Первый пункт я проверил построив картинку, согласно ей ничего неправильного в алгоритме нет. [url=http://funkyimg.com/i/2Neqw.png](ссылка на построенную картинку)[/url]
Второй пункт проверял через boost::multiprecision::cpp_bin_float_quad - то есть число четверной точности. И ответ всё-равно остается такой.
Третий пункт тоже отпадает, потому что на рисунке точка находится внутри полигона.

Предполагаю что при составлении тестов нашлась не самая оптимальная точка. 
Прошу проверить эту точку и сообщить результаты.
*/
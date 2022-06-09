
%{
% Anti-notch IIR filter.
% It is used to emphasize the 3-base periodicity by making peaks
% in a digitized DNA sequence.
%}

%{ 
% Input data vector
% It can be a digitized DNA sequence.
%}
input_vec = [ 0.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0 ];

%{ The pole radius R. %}
R = 0.992;

%{ The filter's center angular frequency θ. %}
theta = ( 2 * pi ) / 3;

%{
% Filter coefficients.
% b = Numerator coefficient
% a = Denominator coefficient
%}
b = [ 1, 0, -1 ];
a = [ 1, -2 * R * cos( theta ), R^2 ];
    
%{ MATLAB filter function %}
output_vec = filter( b, a, input_vec );

disp( round( output_vec, 3 ) )

%{
% >> biquad_filter
%          0    1.0000    0.0080   -0.9920    0.9760    0.0080   -0.9680    0.9530    0.0080   -1.9450    0.9220    0.9990
%
%}

clc
clear
%f_2=@(x) x.^7+8*x.^6-45*x.^5+56*x.^3-30*x+64;
f3=@(x) 7*x.^3-49*x.^2+29*x-2
df3=@(z) 21*z.^2-98*z+29

f4=@(z3) e.^(2*z3-9*tan(z3/pi))-9*sin(z3)
df4=@(z4) (2-(9/pi)*(sec(z4/pi)).^2)*e.^(2*z4-9*tan(z4/pi))-9*cos(z4)

phix=@(z2) (-7*z2.^3+49*z2.^2+2)/29




function y=raiz(method, f1, f2, x)
   switch method
     case 1
        y=f1(x)
     case 2
        y=x-(f1(x)/f2(x))
   endswitch
endfunction



i=-10
j=-5.7
h=0.05

vx=i:h:j;
vy=f4(vx);

plot(vx, vy, 'b-', 'LineWidth',2);
grid on;
hold on;

tic();
start=tic();

i=-7
j=-5.6 %valores dentro do range a buscar a raiz

k=0;
prec=10.^-3;
method=2

xnew=raiz(method, f4, df4, i);
hist_x=[i, xnew]; % histórico de valores para encontrar o zero da função
hist_y=[f4(i), f4(xnew)];
printf("k=%d, x=%.5f, f(x)=%g \n\n", k, xnew, f4(xnew));

while(abs(f4(xnew))>prec) % verificamos a precisão para o novo x
  x=xnew;
  xnew=raiz(method, f4, df4, x);
  k++;
  hist_x(end+1)=xnew;
  hist_y(end+1)=f4(xnew);
  printf("k=%d, x=%.5f, f(x)=%g \n\n", k, xnew, f4(xnew));
endwhile

x=xnew;

plot(hist_x, hist_y, 'm--o', 'MarkerSize', 6, 'LineWidth', 1);
plot(x,f4(x),'o', 'Color', 'r', 'MarkerSize', 10, 'LineWidth', 1.3);
hold off;

#{


#}

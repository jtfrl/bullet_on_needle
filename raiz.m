clc
clear %apaga instruçõies anteriores no terminal

f=@(x) x.^3-9*x+5;

function y=raiz(opc, a, b)
  switch opc
    case 1
      y=(a+b)/2;
    case 2
      y=(a*f(b)-b*f(a))/(f(b)-f(a));
    %case 3
	  %  y=
    otherwise
      y=NaN;
  endswitch
endfunction

a=-4;
b=-a;
h=1;

vx=a:h:b;
vy=f(vx);

plot(vx, vy, 'b-', 'LineWidth',2);
grid on;
hold on;

tic(); %mostra tempo em segundos para execução
start=tic();
a=0
b=1
prec=10.^-2
k=0;
metodo=2 % 1 = bisseccao | 2 = falsa posicao

x=raiz(metodo, f, a, b);
%x=(a+b)/2;

printf("k=%d, x=%.5f, f(x)=%g\n", k, x, f(x));

while(abs(f(x))>prec) %enquanto for maior que a precisao
  if(f(a)*f(x) < 0)
    b=x;
  else
    a=x;
  endif
  k++;
  x=(a+b)/2;
  printf("k=%d, x=%.5f, f(x)=%g\n", k, x, f(x));
endwhile

plot(x,f(x),'o', 'Color', 'r', 'MarkerSize', 10);
hold off;

elapsed_time = toc(start);
printf("Execution time: %.4f seconds\n", elapsed_time);
%print(toc());

type DebouncedFunction<F extends (...args: any[]) => void> = (this: ThisParameterType<F>, ...args: Parameters<F>) => void;

export function debounce<F extends (...args: any[]) => void>(func: F, delay: number=300): DebouncedFunction<F> {
  let timerId: ReturnType<typeof setTimeout> | null;
  
  return function(this: ThisParameterType<F>, ...args: Parameters<F>) {
    if (timerId) {
      clearTimeout(timerId);
    }
    
    timerId = setTimeout(() => {
      func.apply(this, args);
      timerId = null;
    }, delay);
  } as DebouncedFunction<F>;
}

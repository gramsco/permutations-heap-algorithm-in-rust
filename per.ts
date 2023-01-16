function swap_in_place(data, indexA, indexB) {
  [data[indexA], data[indexB]] = [data[indexB], data[indexA]];
}

function heap_permute(data, size, callback) {
  if (size === 1) {
    callback(data);
    return;
  }

  for (let i = 0; i < size; i++) {
    heap_permute(data, size - 1, callback);
    let s1 = size & 1 ? i : 0;
    swap_in_place(data, s1, size - 1);
  }
}

function main() {
  const arr = ["a", "b"];
  heap_permute(arr, arr.length, console.log);
}

main();

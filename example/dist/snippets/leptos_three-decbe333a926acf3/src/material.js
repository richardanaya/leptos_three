export function attachMaterial(object3d, r, g, b) {
  // color is from 0..1
  // create material from color and set object3d material
  const material = new THREE.MeshBasicMaterial({
    color: new THREE.Color(r, g, b),
  });
  object3d.material = material;
}

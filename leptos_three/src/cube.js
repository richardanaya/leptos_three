export function createCube(scene, x, y, z) {
  // add cube to scene
  const geometry = new THREE.BoxGeometry();
  const material = new THREE.MeshBasicMaterial();
  const cube = new THREE.Mesh(geometry, material);
  cube.position.x = x;
  cube.position.y = y;
  scene.add(cube);

  // animate
  const animate = () => {
    requestAnimationFrame(animate);
    cube.rotation.x += 0.01;
    cube.rotation.y += 0.01;
  };
  animate();

  return cube;
}

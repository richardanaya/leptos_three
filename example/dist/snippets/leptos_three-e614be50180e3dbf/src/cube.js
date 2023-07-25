export function attachToScene(scene, mesh) {
  debugger;
  // add cube to scene
  const cube = mesh;
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

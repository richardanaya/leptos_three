export function startCanvas(renderCanvas, camera, scene) {
  const renderer = new THREE.WebGLRenderer({ canvas: renderCanvas });
  renderer.setSize(renderCanvas.clientWidth, renderCanvas.clientHeight);
  renderer.render(scene, camera);

  const animate = () => {
    requestAnimationFrame(animate);
    renderer.render(scene, camera);
  };
  animate();

  // add resize watch of renderCanvas
  const resizeObserver = new ResizeObserver((entries) => {
    for (let entry of entries) {
      const { width, height } = entry.contentRect;
      renderer.setSize(width, height);
      camera.aspect = width / height;
      camera.updateProjectionMatrix();
    }
  });
  resizeObserver.observe(renderCanvas);
  return scene;
}

using DlclRpc;

namespace DlclNet.Models;

public struct Animation : IDTODerivable<AnimationDTO>
{
    public uint Layer { get; set; }
    public List<Frame> Frames { get; set; }
    
    public AnimationDTO ToDTO()
    {
        var dto = new AnimationDTO();
        dto.Layer = Layer;

        var frame_dtos = Frames.ConvertAll(frame => frame.ToDTO());
        dto.Frames.AddRange(frame_dtos);

        return dto;
    }
}